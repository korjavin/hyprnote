// https://api.ncloud-docs.com/docs/en/ai-application-service-clovaspeech-grpc

pub mod interface;

use anyhow::Result;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};

use interface::nest_service_client::NestServiceClient;
use tonic::{service::interceptor::InterceptedService, transport::Channel, Request, Status};

// https://docs.rs/tonic/latest/tonic/service/trait.Interceptor.html
// 'Send' is required in the websocket handler context
type Interceptor = Box<dyn FnMut(Request<()>) -> Result<Request<()>, Status> + Send>;

#[derive(Debug)]
pub struct Client {
    inner: NestServiceClient<InterceptedService<Channel, Interceptor>>,
    config: interface::ConfigRequest,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    keywords: Option<Vec<String>>,
}

impl ClientBuilder {
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn keywords(mut self, keywords: impl Into<Vec<String>>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub async fn build(self) -> Result<Client, crate::Error> {
        let channel =
            tonic::transport::Channel::from_static("https://clovaspeech-gw.ncloud.com:50051")
                .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())?
                .connect()
                .await?;

        let inner = NestServiceClient::with_interceptor(
            channel,
            Self::make_interceptor(self.api_key.unwrap()),
        );

        let config = interface::ConfigRequest {
            transcription: Some(interface::Transcription {
                language: interface::Language::Korean,
            }),
            keyword_boosting: Some(self.keywords.unwrap_or_default().into()),
            semantic_epd: Some(interface::SemanticEpd {
                skip_empty_text: Some(true),
                use_word_epd: Some(true),
                gap_threshold: Some(500),
                ..Default::default()
            }),
        };

        Ok(Client { inner, config })
    }

    fn make_interceptor(secret_key: String) -> Interceptor {
        Box::new(move |mut req: Request<()>| {
            req.metadata_mut()
                // lowercase is required
                .insert("authorization", secret_key.parse().unwrap());
            Ok(req)
        })
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn from_audio<S, E>(
        &mut self,
        audio: S,
    ) -> Result<impl Stream<Item = Result<interface::StreamResponse, crate::Error>>, crate::Error>
    where
        S: Stream<Item = Result<Bytes, E>> + Send + Unpin + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let config = serde_json::to_string(&self.config).unwrap();
        let config_request = interface::NestRequest {
            r#type: interface::RequestType::Config.into(),
            part: Some(interface::nest_request::Part::Config(
                interface::NestConfig { config },
            )),
        };
        let config_stream = futures_util::stream::once(async move { config_request });

        let audio_request_stream = audio.filter_map(|chunk| async {
            chunk.ok().map(|chunk| interface::NestRequest {
                r#type: interface::RequestType::Data.into(),
                part: Some(interface::nest_request::Part::Data(interface::NestData {
                    chunk: chunk.into(),
                    extra_contents: serde_json::to_string(
                        &interface::RecognizeRequestExtra::default(),
                    )
                    .unwrap(),
                })),
            })
        });

        let response = self
            .inner
            .recognize(config_stream.chain(audio_request_stream))
            .await?
            .into_inner()
            .map(|message| {
                let res = serde_json::from_str::<interface::StreamResponse>(&message?.contents)?;
                Ok(res)
            });

        Ok(response)
    }
}

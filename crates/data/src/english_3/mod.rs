pub const AUDIO: &[u8] = include_wav!("./audio.wav");

pub const AUDIO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/english_3/audio.wav");

pub const TRANSCRIPTION_JSON: &str = include_str!("./transcription.json");

pub const TRANSCRIPTION_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/english_3/transcription.json"
);

pub const DIARIZATION_JSON: &str = include_str!("./diarization.json");

pub const DIARIZATION_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/english_3/diarization.json"
);

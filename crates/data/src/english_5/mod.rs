pub const TRANSCRIPTION_JSON: &str = include_str!("./transcription.json");

pub const TRANSCRIPTION_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/english_5/transcription.json"
);

pub const DIARIZATION_JSON: &str = include_str!("./diarization.json");

pub const DIARIZATION_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/english_5/diarization.json"
);

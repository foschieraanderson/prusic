use std::path::PathBuf;

pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub path: PathBuf,
    pub cover: Option<Vec<u8>>,
}

impl Song {
    pub fn from_path(path: PathBuf) -> Self {
        let title = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            title,
            artist: "Unknown artist".to_string(),
            album: "Unknown album".to_string(),
            path,
            cover: None,
        }
    }
}

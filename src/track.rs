use id3::{Tag, TagLike};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<i32>,
    pub cover: Option<Vec<u8>>,
}

impl Track {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let tag = Tag::read_from_path(&path).ok();

        let title = tag
            .as_ref()
            .and_then(|tag| tag.title())
            .filter(|title| !title.trim().is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown")
                    .to_string()
            });

        let artist = tag
            .as_ref()
            .and_then(|tag| tag.artist())
            .unwrap_or("Unknown")
            .to_string();

        let album = tag
            .as_ref()
            .and_then(|tag| tag.album())
            .unwrap_or("Unknown")
            .to_string();

        let year = tag.as_ref().and_then(|tag| tag.year());

        let cover = tag
            .as_ref()
            .and_then(|tag| tag.pictures().next())
            .map(|picture| picture.data.clone());

        Ok(Self {
            path,
            title,
            artist,
            album,
            year,
            cover,
        })
    }
}

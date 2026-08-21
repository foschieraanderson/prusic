use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::song::Song;

pub fn scan_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() && is_audio_file(&path) {
            files.push(path);
        }
    }

    files.sort();

    Ok(files)
}

pub fn scan_songs(path: &Path) -> io::Result<Vec<Song>> {
    let files = scan_directory(path)?;

    let songs = files.into_iter().map(Song::from_path).collect();

    Ok(songs)
}

fn is_audio_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "mp3" | "flac" | "ogg" | "wav" | "m4a"
            )
        }

        None => false,
    }
}

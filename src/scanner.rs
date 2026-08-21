use std::{
    fs, io,
    path::{Path, PathBuf},
};

use lofty::{file::TaggedFileExt, read_from_path, tag::Accessor};

use crate::song::{Cover, Song};

pub fn scan_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    scan_directory_recursive(path, &mut files)?;

    files.sort();

    Ok(files)
}

fn scan_directory_recursive(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let path = entry.path();

        if path.is_dir() {
            scan_directory_recursive(&path, files)?;
        } else if path.is_file() && is_audio_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

pub fn scan_songs(path: &Path) -> io::Result<Vec<Song>> {
    let files = scan_directory(path)?;

    let songs = files.into_iter().map(|path| load_metadata(&path)).collect();

    Ok(songs)
}

fn load_metadata(path: &Path) -> Song {
    let mut song = Song::from_path(path.to_path_buf());

    let Ok(tagged_file) = read_from_path(path) else {
        return song;
    };

    let Some(tag) = tagged_file.primary_tag() else {
        return song;
    };

    if let Some(title) = tag.title() {
        song.title = title.to_string();
    }

    if let Some(artist) = tag.artist() {
        song.artist = artist.to_string();
    }

    if let Some(album) = tag.album() {
        song.album = album.to_string();
    }

    if let Some(picture) = tag.pictures().first() {
        let mime_type = picture
            .mime_type()
            .map(|mime| mime.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        song.cover = Some(Cover {
            mime_type,
            data: picture.data().to_vec(),
        });
    }

    song
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

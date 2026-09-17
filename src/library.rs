use ratatui::widgets::ListState;

use crate::helpers::is_audio_file;
use crate::track::Track;

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

pub enum LibraryNode {
    Directory {
        path: PathBuf,
        name: String,
        children: Vec<LibraryNode>,
    },
    Track(Track),
}

pub struct Library {
    pub root: LibraryNode,
}

pub struct LibraryState {
    pub expanded: HashSet<PathBuf>,
    pub selected_tracks: HashSet<PathBuf>,
    pub list_state: ListState,
}

impl Library {
    pub fn from_directory(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let root = Self::scan_directory(path)?;

        Ok(Self { root })
    }

    fn scan_directory(path: &Path) -> Result<LibraryNode, Box<dyn std::error::Error>> {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("/")
            .to_string();

        let mut children = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                children.push(Self::scan_directory(&path)?);
            } else if path.is_file() && is_audio_file(&path) {
                match Track::new(path) {
                    Ok(track) => children.push(LibraryNode::Track(track)),
                    Err(e) => {
                        eprintln!("Erro ao carregar música: {e}");
                    }
                }
            }
        }

        children.sort_by(|a, b| {
            let name_a = match a {
                LibraryNode::Directory { name, .. } => name,
                LibraryNode::Track(track) => &track.title,
            };

            let name_b = match b {
                LibraryNode::Directory { name, .. } => name,
                LibraryNode::Track(track) => &track.title,
            };

            name_a.to_lowercase().cmp(&name_b.to_lowercase())
        });

        Ok(LibraryNode::Directory {
            path: path.to_path_buf(),
            name,
            children,
        })
    }
}

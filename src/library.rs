use ratatui::widgets::ListState;

use crate::track::Track;
use crate::{helpers::is_audio_file, playlist::Playlist};

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
    pub state: LibraryState,
}

pub struct LibraryState {
    pub expanded: HashSet<PathBuf>,
    pub selected_tracks: HashSet<PathBuf>,
    pub list_state: ListState,
}

impl Library {
    pub fn from_directory(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let root = Self::scan_directory(path)?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            root,
            state: LibraryState {
                expanded: HashSet::new(),
                selected_tracks: HashSet::new(),
                list_state,
            },
        })
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

    pub fn visible_nodes(&self) -> Vec<(&LibraryNode, usize)> {
        let mut nodes = Vec::new();

        Self::collect_visible(&self.root, &self.state, &mut nodes, 0);

        nodes
    }

    fn collect_visible<'a>(
        node: &'a LibraryNode,
        state: &LibraryState,
        nodes: &mut Vec<(&'a LibraryNode, usize)>,
        depth: usize,
    ) {
        let LibraryNode::Directory { children, .. } = node else {
            return;
        };

        for child in children {
            nodes.push((child, depth));

            if let LibraryNode::Directory { path, .. } = child {
                if state.expanded.contains(path) {
                    Self::collect_visible(child, state, nodes, depth + 1);
                }
            }
        }
    }

    pub fn select_next(&mut self) {
        let nodes = self.visible_nodes();

        if nodes.is_empty() {
            return;
        }

        let next = match self.state.list_state.selected() {
            Some(i) => {
                if i + 1 < nodes.len() {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };

        self.state.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        let nodes = self.visible_nodes();

        if nodes.is_empty() {
            return;
        }

        let previous = match self.state.list_state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };

        self.state.list_state.select(Some(previous));
    }

    pub fn selected_node(&self) -> Option<&LibraryNode> {
        let index = self.state.list_state.selected()?;

        self.visible_nodes().get(index).map(|(node, _)| *node)
    }

    pub fn toggle_selected(&mut self) {
        let Some(index) = self.state.list_state.selected() else {
            return;
        };

        let path = {
            let nodes = self.visible_nodes();

            let Some((node, _)) = nodes.get(index) else {
                return;
            };

            let LibraryNode::Directory { path, .. } = node else {
                return;
            };

            path.clone()
        };

        if !self.state.expanded.insert(path.clone()) {
            self.state.expanded.remove(&path);
        }
    }

    pub fn toggle_track_selection(&mut self) {
        let Some(index) = self.state.list_state.selected() else {
            return;
        };

        let path = {
            let nodes = self.visible_nodes();

            let Some((node, _)) = nodes.get(index) else {
                return;
            };

            let LibraryNode::Track(track) = node else {
                return;
            };

            track.path.clone()
        };

        if !self.state.selected_tracks.insert(path.clone()) {
            self.state.selected_tracks.remove(&path);
        }
    }

    pub fn selected_tracks(&self) -> Vec<Track> {
        let mut tracks = Vec::new();

        Self::collect_selected_tracks(&self.root, &self.state.selected_tracks, &mut tracks);

        tracks
    }

    fn collect_selected_tracks(
        node: &LibraryNode,
        selected: &HashSet<PathBuf>,
        tracks: &mut Vec<Track>,
    ) {
        match node {
            LibraryNode::Directory { children, .. } => {
                for child in children {
                    Self::collect_selected_tracks(child, selected, tracks);
                }
            }

            LibraryNode::Track(track) => {
                if selected.contains(&track.path) {
                    tracks.push(track.clone());
                }
            }
        }
    }

    pub fn add_selected_to_playlist(&mut self, playlist: &mut Playlist) {
        let tracks = self.selected_tracks();

        for track in tracks {
            playlist.add_track(track);
        }
    }
}

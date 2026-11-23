
use std::path::PathBuf;

pub enum RepeatMode {
    Once,
    Playlist,
    RepeatOne,
    RepeatPlaylist,
}

pub struct Playlist(Vec<PathBuf>);

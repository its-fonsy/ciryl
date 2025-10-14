use crate::cmus::{Cmus, PlayerSongInfo};
use crate::gui::Gui;
use crate::lyric::Lyric;

#[derive(PartialEq, Clone, Copy)]
pub enum RuntimeError {
    ErrorSocketConnect,
    ErrorSocketRead,
    ErrorSocketWrite,
    ErrorExpectedNumber,
    ErrorEnvironmentVariableNotSet,
    ErrorLyricNotFound,
    None,
}

#[derive(PartialEq)]
pub enum RuntimeStatus {
    NewSong,
    NewIndex,
    NoUpdate,
}

pub struct RuntimeContext {
    pub player: Cmus,
    pub lyric: Lyric,
    pub song: PlayerSongInfo,
    pub fixed_index: usize,
    pub state: RuntimeStatus,
}

impl RuntimeContext {
    pub fn new() -> RuntimeContext {
        RuntimeContext {
            player: Cmus::new(),
            lyric: Lyric::new(),
            song: PlayerSongInfo::new(),
            fixed_index: 0,
            state: RuntimeStatus::NoUpdate,
        }
    }

    pub fn init(&mut self) {
        Gui::initialize().unwrap_or_else(|error| {
            eprintln!("Error initializing the GUI: {}", error);
            std::process::exit(1);
        });
    }

    pub fn update(&mut self) -> Result<(), RuntimeError> {
        self.state = RuntimeStatus::NoUpdate;

        self.player.update()?;

        let song = self.player.playing_song_metadata()?;

        if self.song != song {
            self.state = RuntimeStatus::NewSong;
            self.song = song.clone();
            self.lyric.parse(&song)?;
            self.fixed_index = self.lyric.get_singed_verse_index(song.position);
            return Ok(());
        }

        let fixed_index = self.lyric.get_singed_verse_index(song.position);
        if fixed_index != self.fixed_index {
            self.fixed_index = fixed_index;
            self.state = RuntimeStatus::NewIndex;
        }

        Ok(())
    }
}

mod cmus;
mod error;
mod gui;
mod lyric;

use cmus::{Cmus, PlayerSongInfo};
use error::RuntimeError;
use gui::Gui;
use lyric::Lyric;

type Result<T> = std::result::Result<T, RuntimeError>;

pub enum RuntimeReturn {
    Continue,
    Exit,
}

#[derive(PartialEq)]
enum RuntimeUpdate {
    NewSong,
    NewIndex,
    Nop,
    LyricNotFound,
    LyricDirNotSet,
    CmusError,
    ParseError,
    DisplayError,
}

pub struct CirylRuntime {
    player: Cmus,
    lyric: Lyric,
    song: PlayerSongInfo,
    fixed_index: usize,
    initialized: bool,
    last_update: RuntimeUpdate,
}

impl CirylRuntime {
    pub fn new() -> CirylRuntime {
        CirylRuntime {
            player: Cmus::new(),
            lyric: Lyric::new(),
            song: PlayerSongInfo::new(),
            fixed_index: 0,
            initialized: false,
            last_update: RuntimeUpdate::Nop,
        }
    }

    fn update(&mut self) -> RuntimeUpdate {
        if let Err(_) = self.player.update() {
            return match self.last_update {
                RuntimeUpdate::DisplayError => RuntimeUpdate::DisplayError,
                _ => RuntimeUpdate::CmusError,
            };
        };

        let song = match self.player.playing_song_metadata() {
            Ok(metadata) => metadata,
            Err(_) => {
                return match self.last_update {
                    RuntimeUpdate::DisplayError => RuntimeUpdate::DisplayError,
                    _ => RuntimeUpdate::ParseError,
                };
            }
        };

        if self.song != song {
            self.song = song.clone();

            match self.lyric.parse(&song) {
                Ok(_) => {}
                Err(RuntimeError::LyricNotFound) => return RuntimeUpdate::LyricNotFound,
                Err(RuntimeError::LyricDirEnvNotSet) => return RuntimeUpdate::LyricDirNotSet,
                Err(_) => return RuntimeUpdate::ParseError,
            };

            self.fixed_index = self.lyric.get_singed_verse_index(song.position);
            return RuntimeUpdate::NewSong;
        }

        if self.last_update == RuntimeUpdate::DisplayError {
            return RuntimeUpdate::DisplayError;
        }

        let fixed_index = self.lyric.get_singed_verse_index(song.position);
        if fixed_index != self.fixed_index {
            self.fixed_index = fixed_index;
            return RuntimeUpdate::NewIndex;
        }

        RuntimeUpdate::Nop
    }

    pub fn task(&mut self) -> Result<RuntimeReturn> {
        if !self.initialized {
            Gui::initialize()?;
            self.initialized = true;
        }

        let update = self.update();

        match update {
            RuntimeUpdate::NewSong => {
                Gui::clear_screen()?;
                // Gui::print_vector(&self.lyric.text, self.fixed_index)?;
            }
            RuntimeUpdate::NewIndex => {},/* )Gui::print_vector(&self.lyric.verses, self.fixed_index)?, */
            RuntimeUpdate::CmusError => Gui::print_general_error("Can't connect to CMUS socket")?,
            RuntimeUpdate::ParseError => Gui::print_general_error("Can't parse playing song")?,
            RuntimeUpdate::LyricDirNotSet => {
                Gui::print_general_error("LYRIC_DIR environment directory not set")?
            }
            RuntimeUpdate::LyricNotFound => {
                Gui::print_lyric_not_found_error(&self.song.artist, &self.song.title)?
            }
            RuntimeUpdate::DisplayError => {}
            RuntimeUpdate::Nop => {}
        }

        self.last_update = match update {
            RuntimeUpdate::CmusError
            | RuntimeUpdate::ParseError
            | RuntimeUpdate::LyricDirNotSet
            | RuntimeUpdate::LyricNotFound
            | RuntimeUpdate::DisplayError => RuntimeUpdate::DisplayError,
            RuntimeUpdate::NewSong => RuntimeUpdate::NewSong,
            RuntimeUpdate::NewIndex => RuntimeUpdate::NewIndex,
            RuntimeUpdate::Nop => RuntimeUpdate::Nop,
        };

        match Gui::pool_keyboard()? {
            Some(key) => {
                match key {
                    /* Press 'q' to quit */
                    'q' => {
                        Gui::terminate()?;
                        return Ok(RuntimeReturn::Exit);
                    },
                    /* Press 'r' to retry song parsing */
                    'r' => {
                        self.song = PlayerSongInfo::new();
                    }
                    _ => {},
                };
            }
            None => {}
        }

        Ok(RuntimeReturn::Continue)
    }
}

use std::error::Error;

mod cmus;
mod error;
mod gui;
mod lyric;

use crate::cmus::{Cmus, PlayerSongInfo};
use crate::error::RuntimeError;
use crate::gui::Gui;
use crate::lyric::Lyric;

#[derive(PartialEq)]
enum RuntimeStatus {
    NewSong,
    NewIndex,
    NoUpdate,
}

struct RuntimeContext {
    player: Cmus,
    lyric: Lyric,
    song: PlayerSongInfo,
    fixed_index: usize,
    state: RuntimeStatus,
}

impl RuntimeContext {
    fn new() -> RuntimeContext {
        RuntimeContext {
            player: Cmus::new(),
            lyric: Lyric::new(),
            song: PlayerSongInfo::new(),
            fixed_index: 0,
            state: RuntimeStatus::NoUpdate,
        }
    }

    fn init(&mut self) {
        Gui::initialize().unwrap_or_else(|error| {
            eprintln!("Error initializing the GUI: {}", error);
            std::process::exit(1);
        });
    }

    fn update(&mut self) -> Result<(), RuntimeError> {
        self.state = RuntimeStatus::NoUpdate;

        self.player.update()?;

        let song = self.player.playing_song_metadata()?;

        if self.song != song {
            self.song = song.clone();
            self.lyric.parse(&song)?;

            self.fixed_index = self.lyric.get_singed_verse_index(song.position);
            self.state = RuntimeStatus::NewSong;
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

fn main() {
    if let Err(err) = run() {
        let _ = Gui::terminate();
        eprintln!("Error: {err}");
        std::process::exit(1);
    };
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut runtime = RuntimeContext::new();
    let mut last_error_msg = String::new();

    runtime.init();

    loop {
        let res = runtime.update();

        match res {
            Ok(_) => {
                if runtime.lyric.valid {
                    match runtime.state {
                        RuntimeStatus::NewSong | RuntimeStatus::NewIndex => {
                            Gui::print_vector(&runtime.lyric.text, runtime.fixed_index)?;
                            last_error_msg = "".to_string();
                        }
                        RuntimeStatus::NoUpdate => (),
                    }
                }
            }
            Err(e) => {
                let msg = match e {
                    RuntimeError::ErrorSocketConnect => "Can't connect to CMUS socket".to_string(),
                    RuntimeError::ErrorSocketRead => "Can't read CMUS socket".to_string(),
                    RuntimeError::ErrorSocketWrite => "Can't write CMUS socket".to_string(),
                    RuntimeError::ErrorExpectedNumber => "Failed parsing song metadata".to_string(),
                    RuntimeError::ErrorEnvironmentVariableNotSet => {
                        "Enviromental variable $LYRIC not set".to_string()
                    }
                    RuntimeError::ErrorFileNotFound => format!(
                        "Lyric \"{} - {}.lrc\" not found",
                        runtime.song.artist, runtime.song.title
                    ),
                };

                if last_error_msg != msg || runtime.state == RuntimeStatus::NewSong {
                    last_error_msg = msg.clone();
                    Gui::print_error(&msg)?;
                }
            }
        }

        match Gui::pool_keyboard()? {
            Some(key) => {
                if key == 'q' {
                    break;
                }
            }
            None => (),
        }
    }
    Gui::terminate()?;

    Ok(())
}

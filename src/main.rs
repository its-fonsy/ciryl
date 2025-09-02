use std::error::Error;

mod cmus;
mod gui;
mod lyric;

use crate::cmus::{Cmus, PlayerSongInfo};
use crate::gui::Gui;
use crate::lyric::Lyric;

#[derive(PartialEq)]
enum RuntimeError {
    PlayerUpdate,
    LyricParse,
    SongMetadata,
    None,
}

struct RuntimeContext {
    player: Cmus,
    lyric: Lyric,
    song: PlayerSongInfo,
    fixed_index: usize,
    valid_lyric: bool,
    print_lyric: bool,
}

impl RuntimeContext {
    fn new() -> RuntimeContext {
        RuntimeContext {
            player: Cmus::new(),
            lyric: Lyric::new(),
            song: PlayerSongInfo::new(),
            fixed_index: 0,
            valid_lyric: false,
            print_lyric: false,
        }
    }

    fn init(&mut self) {
        Gui::initialize().unwrap_or_else(|error| {
            eprintln!("Error initializing the GUI: {}", error);
            std::process::exit(1);
        });
    }

    fn update(&mut self) -> Result<(), RuntimeError> {
        self.print_lyric = false;

        self.player
            .update()
            .map_err(|_| RuntimeError::PlayerUpdate)?;

        let song = self
            .player
            .playing_song_metadata()
            .map_err(|_| RuntimeError::SongMetadata)?;

        if self.song != song {
            self.valid_lyric = false;

            self.song = song.clone();
            self.lyric
                .parse(&song)
                .map_err(|_| RuntimeError::LyricParse)?;

            self.valid_lyric = true;
            self.print_lyric = true;
            self.fixed_index = self.lyric.get_singed_verse_index(song.position);
            return Ok(());
        }

        let fixed_index = self.lyric.get_singed_verse_index(song.position);
        if fixed_index != self.fixed_index {
            self.fixed_index = fixed_index;
            self.print_lyric = true;
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
    let mut last_error = RuntimeError::None;

    runtime.init();

    loop {
        match runtime.update() {
            Ok(_) => {
                if runtime.valid_lyric && runtime.print_lyric {
                    Gui::print_vector(&runtime.lyric.text, runtime.fixed_index)?;
                    last_error = RuntimeError::None;
                }
            }
            Err(e) => {
                let msg = match e {
                    RuntimeError::PlayerUpdate => "Player update error".to_string(),
                    RuntimeError::LyricParse => "Error parsing the lyric".to_string(),
                    RuntimeError::SongMetadata => "Error parsing song metadata".to_string(),
                    _ => "Unknowkn error".to_string(),
                };
                if last_error != e {
                    last_error = e;
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

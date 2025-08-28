use std::error::Error;
use std::process;

mod cmus;
mod gui;
mod lyric;

use crate::cmus::Cmus;
use crate::gui::Gui;
use crate::lyric::{Lyric, LyricState};

fn main() {
    if let Err(err) = run() {
        let _ = Gui::terminate();
        eprintln!("Error: {err}");
        process::exit(1);
    };
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut player = Cmus::new()?;
    let mut lyric = Lyric::new();
    let mut vector: Vec<&str> = Vec::new();
    let mut fixed_index;
    let mut prev_fixed_index = 9999;

    Gui::initialize()?;
    loop {
        player.update()?;

        if player.playing_song_metadata()? != lyric.song {
            vector.clear();
            lyric.song = player.playing_song_metadata()?;
            if let Err(_) = lyric.get_lyric() {
                lyric.state = LyricState::LyricNotFound;
                let _ = Gui::print_error(&format!(
                    "Lyric not found for \"{} - {}\"",
                    lyric.song.artist, lyric.song.title
                ));
            };
            vector = lyric.verses.iter().map(|s| s.text.as_str()).collect();
        }

        if lyric.state == LyricState::ValidLyric {
            lyric.song = player.playing_song_metadata()?;
            fixed_index = lyric.get_singed_verse_index();

            if fixed_index != prev_fixed_index {
                prev_fixed_index = fixed_index;
                Gui::print_vector(&vector, fixed_index)?;
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

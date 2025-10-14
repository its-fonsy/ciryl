use std::error::Error;

mod cmus;
mod gui;
mod lyric;
mod runtime;

use crate::gui::Gui;
use crate::runtime::{RuntimeContext, RuntimeError, RuntimeStatus};

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
        let res = runtime.update();

        match res {
            Ok(_) => {
                if runtime.lyric.valid {
                    match runtime.state {
                        RuntimeStatus::NewSong | RuntimeStatus::NewIndex => {
                            if runtime.state == RuntimeStatus::NewSong {
                                Gui::clear_screen()?;
                            }
                            Gui::print_vector(&runtime.lyric.text, runtime.fixed_index)?;
                            last_error = RuntimeError::None;
                        }
                        RuntimeStatus::NoUpdate => (),
                    }
                }
            }
            Err(e) => {
                if last_error != e || runtime.state == RuntimeStatus::NewSong {
                    last_error = e;
                    match e {
                        RuntimeError::ErrorSocketConnect => {
                            Gui::print_general_error("Can't connect to CMUS socket")?
                        }
                        RuntimeError::ErrorSocketRead => {
                            Gui::print_general_error("Can't read CMUS socket")?
                        }
                        RuntimeError::ErrorSocketWrite => {
                            Gui::print_general_error("Can't write CMUS socket")?
                        }
                        RuntimeError::ErrorExpectedNumber => {
                            Gui::print_general_error("Failed parsing song metadata")?
                        }
                        RuntimeError::ErrorEnvironmentVariableNotSet => {
                            Gui::print_general_error("Enviromental variable $LYRIC not set")?
                        }
                        RuntimeError::ErrorLyricNotFound => Gui::print_lyric_not_found_error(
                            &runtime.song.artist,
                            &runtime.song.title,
                        )?,
                        _ => (),
                    };
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

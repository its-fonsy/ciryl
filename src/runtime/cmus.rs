use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use crate::runtime::RuntimeError;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Clone)]
pub struct PlayerSongInfo {
    pub title: String,
    pub artist: String,
    pub position: usize,
}

impl PlayerSongInfo {
    pub fn new() -> PlayerSongInfo {
        PlayerSongInfo {
            title: String::new(),
            artist: String::new(),
            position: 0,
        }
    }
}

impl PartialEq for PlayerSongInfo {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.artist == other.artist
    }

    fn ne(&self, other: &Self) -> bool {
        self.title != other.title || self.artist != other.artist
    }
}

impl Eq for PlayerSongInfo {}

pub struct Cmus {
    socket_path: String,
    status: String,
}

const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

impl Cmus {
    pub fn new() -> Cmus {
        let mut socket_path: String = match env::var(XDG_RUNTIME_DIR) {
            Ok(path) => path,
            Err(error) => {
                eprintln!("Error reading environment variable: {}", error);
                std::process::exit(1);
            }
        };
        let status = String::new();
        socket_path.push_str("/cmus-socket");

        Cmus {
            socket_path,
            status,
        }
    }

    pub fn playing_song_metadata(&self) -> Result<PlayerSongInfo> {
        let title = self.parse_status("tag title");
        let artist = self.parse_status("tag artist");
        let position: usize = self.parse_status("position").parse()?;
        let position = position * 1000;

        Ok(PlayerSongInfo {
            title,
            artist,
            position,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        let mut response = [0; 2048];
        let mut stream = UnixStream::connect(self.socket_path.clone())?;

        stream.write(b"status\n")?;
        stream.read(&mut response)?;

        let mut i = 0;
        while response[i] != 0 {
            i = i + 1;
        }

        let response = String::from_utf8_lossy(&response[0..i]).to_string();
        self.status = response;
        Ok(())
    }

    fn parse_status(&self, pattern: &str) -> String {
        let mut value = String::new();

        for line in self.status.lines() {
            match line.strip_prefix(pattern) {
                Some(stripped) => {
                    value = String::from(stripped);
                    break;
                }
                None => continue,
            }
        }

        value.trim().to_string()
    }
}

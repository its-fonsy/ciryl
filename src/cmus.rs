use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub struct PlayerSongInfo {
    pub title: String,
    pub artist: String,
    pub position: usize,
}

pub struct Cmus {
    socket_path: String,
    status: String,
}

const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

impl PartialEq for PlayerSongInfo {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.artist == other.artist
    }
}

impl Eq for PlayerSongInfo {}

impl Cmus {
    pub fn new() -> Result<Cmus, Box<dyn Error>> {
        let mut socket_path: String = env::var(XDG_RUNTIME_DIR)?;
        let status = String::new();
        socket_path.push_str("/cmus-socket");

        Ok(Cmus {
            socket_path,
            status,
        })
    }

    pub fn playing_song_metadata(&self) -> Result<PlayerSongInfo, Box<dyn Error>> {
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

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let mut response = [0; 2048];
        let mut stream = UnixStream::connect(self.socket_path.clone())?;

        stream.write(b"status\n").unwrap();
        stream.read(&mut response).unwrap();

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

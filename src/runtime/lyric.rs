use md5;
use regex::Regex;
use std::env;
use std::fs::read_to_string;

use crate::runtime::RuntimeError;
use crate::runtime::cmus::PlayerSongInfo;

type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Lyric {
    timestamps: Vec<usize>,
    pub text: Vec<String>,
}

impl Lyric {
    pub fn new() -> Self {
        Lyric {
            timestamps: Vec::new(),
            text: Vec::new(),
        }
    }

    fn parse_timestamp(timestamp: &str) -> Result<usize> {
        let mut minutes: usize = timestamp[1..3].parse()?;
        let mut seconds: usize = timestamp[4..6].parse()?;
        let mut milli: usize = timestamp[7..9].parse()?;

        minutes = minutes * 60 * 1000;
        seconds = seconds * 1000;
        milli = milli * 10;

        /* Timestamp in milliseconds */

        Ok(minutes + seconds + milli)
    }

    pub fn parse(&mut self, song: &PlayerSongInfo) -> Result<()> {
        let lyric_folder = env::var("LYRICS_DIR")
            .map_err(|_| RuntimeError::LyricDirEnvNotSet)?
            .trim_end_matches('/')
            .to_string();

        let digest = md5::compute(format!("{}{}", song.artist, song.title).as_bytes());

        let filename = format!("{:x}.lrc", digest);
        let filepath = lyric_folder + "/" + &filename;

        /* Parse the file */

        let file_content = match read_to_string(filepath) {
            Ok(content) => content,
            Err(_) => return Err(RuntimeError::LyricNotFound),
        };

        let timestamp_regex = Regex::new(r"^\[\d{2}:\d{2}\.\d{2}]").unwrap();

        self.timestamps.clear();
        self.text.clear();
        for line in file_content.lines() {
            let timestamp = match timestamp_regex.find(line) {
                None => continue,
                Some(ts) => Lyric::parse_timestamp(ts.as_str())?,
            };

            let text = line[10..].trim().to_string();

            self.timestamps.push(timestamp);
            self.text.push(text);
        }

        Ok(())
    }

    pub fn get_singed_verse_index(&self, position: usize) -> usize {
        let mut i = 0;
        for ts in &self.timestamps {
            if position < *ts {
                if i == 0 {
                    break;
                }
                i = i - 1;
                break;
            }
            i = i + 1;
        }
        i
    }
}

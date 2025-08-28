use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

use crate::cmus::PlayerSongInfo;

pub struct Verse {
    pub timestamp: usize,
    pub text: String,
}

#[derive(PartialEq)]
pub enum LyricState {
    NotParsed,
    ValidLyric,
    LyricNotFound,
}

pub struct Lyric {
    pub song: PlayerSongInfo,
    pub verses: Vec<Verse>,
    pub state: LyricState,
}

impl Lyric {
    pub fn new() -> Self {
        let playing_song = PlayerSongInfo {
            title: String::new(),
            artist: String::new(),
            position: 0,
        };
        let lyric: Vec<Verse> = Vec::new();
        Lyric {
            song: playing_song,
            verses: lyric,
            state: LyricState::NotParsed,
        }
    }

    fn parse_timestamp(timestamp: &str) -> Result<usize, Box<dyn Error>> {
        let mut minutes: usize = timestamp[1..3].parse()?;
        let mut seconds: usize = timestamp[4..6].parse()?;
        let mut milli: usize = timestamp[7..9].parse()?;

        minutes = minutes * 60 * 1000;
        seconds = seconds * 1000;
        milli = milli * 10;

        /* Timestamp in milliseconds */

        Ok(minutes + seconds + milli)
    }

    fn parse(&mut self, lrc_file_content: String) -> Result<(), Box<dyn Error>> {
        let timestamp_regex = Regex::new(r"^\[\d{2}:\d{2}\.\d{2}]")?;
        self.verses.clear();
        for line in lrc_file_content.lines() {
            let timestamp = match timestamp_regex.find(line) {
                None => continue,
                Some(ts) => Lyric::parse_timestamp(ts.as_str())?,
            };

            let text = line[10..].trim().to_string();

            let verse = Verse { timestamp, text };
            self.verses.push(verse);
        }

        Ok(())
    }

    fn get_lyric_filepath(&self) -> Result<String, Box<dyn Error>> {
        let lyric_folder = env::var("LYRICS_DIR")?.trim_end_matches('/').to_string();

        let filepath = String::from(format!("{} - {}.lrc", self.song.artist, self.song.title));

        Ok(lyric_folder + "/" + &filepath)
    }

    pub fn get_lyric(&mut self) -> Result<(), Box<dyn Error>> {
        let filepath = Lyric::get_lyric_filepath(&self)?;
        let file_content = read_to_string(filepath)?;
        Lyric::parse(self, file_content)?;
        self.state = LyricState::ValidLyric;
        Ok(())
    }

    pub fn get_singed_verse_index(&self) -> usize {
        let mut i = 0;
        for verse in &self.verses {
            if self.song.position < verse.timestamp {
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

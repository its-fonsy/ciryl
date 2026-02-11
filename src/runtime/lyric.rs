use md5;
use std::env;
use std::fs::read_to_string;

use crate::runtime::RuntimeError;
use crate::runtime::cmus::PlayerSongInfo;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct Verse {
    pub timestamp: usize,
    pub text: String,
}

pub struct Lyric {
    verses: Vec<Verse>,
}

enum LineParseState {
    ParseInit,
    InsideSquareBracket,
    OutsideSquareBracket,
}

impl Lyric {
    pub fn new() -> Lyric {
        Lyric { verses: Vec::new() }
    }

    pub fn get_text(&self) -> Vec<&str> {
        self.verses.iter().map(|v| v.text.as_str()).collect()
    }

    fn parse_timestamp(timestamp: &str) -> Result<usize> {
        let mut minutes: usize = timestamp[0..2].parse()?;
        let mut seconds: usize = timestamp[3..5].parse()?;
        let mut milli: usize = timestamp[6..8].parse()?;

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

        self.verses.clear();

        for line in file_content.lines() {
            self.parse_line(line);
        }

        self.verses.sort();

        Ok(())
    }

    fn parse_line_timestamps(line: &str) -> Vec<usize> {
        let mut state: LineParseState = LineParseState::ParseInit;
        let mut buff: String = String::new();
        let mut timestamps: Vec<usize> = Vec::new();

        buff.clear();
        for char in line.chars() {
            match state {
                LineParseState::ParseInit => {
                    if char == '[' {
                        state = LineParseState::InsideSquareBracket;
                    } else {
                        break;
                    };
                }
                LineParseState::InsideSquareBracket => {
                    if char != ']' {
                        buff.push(char);
                    } else {
                        match Lyric::parse_timestamp(buff.as_str()) {
                            Ok(res) => {
                                timestamps.push(res);
                            }
                            Err(_) => {}
                        };
                        buff.clear();
                        state = LineParseState::OutsideSquareBracket;
                    }
                }
                LineParseState::OutsideSquareBracket => {
                    if char == '[' {
                        state = LineParseState::InsideSquareBracket;
                    }
                }
            };
        }

        timestamps
    }

    fn parse_line_text(line: &str) -> &str {
        let text_begin: usize = match line.rfind(']') {
            Some(index) => index + 1,
            None => return "",
        };

        let verse_text: &str = if text_begin < line.len() {
            line[text_begin..].trim()
        } else {
            ""
        };

        verse_text
    }

    fn parse_line(&mut self, line: &str) {
        let line: &str = line.trim();

        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);

        for timestamp in timestamps {
            let verse = Verse {
                timestamp,
                text: String::from(verse_text),
            };
            self.verses.push(verse);
        }
    }

    pub fn get_singed_verse_index(&self, position: usize) -> usize {
        let mut i = 0;
        for verse in &self.verses {
            let ts: usize = verse.timestamp;
            if position < ts {
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

#[cfg(test)]
mod tests {

    use crate::runtime::lyric::Lyric;

    #[test]
    fn single_timestamp() {
        let line: &str = "[00:34.88] This is a verse";
        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);

        assert_eq!(timestamps, vec![34880]);
        assert_eq!(verse_text, "This is a verse");
    }

    #[test]
    fn single_character() {
        let line: &str = "[00:34.88]    O   ";
        let verse_text: &str = Lyric::parse_line_text(line);
        assert_eq!(verse_text, "O");
    }

    #[test]
    fn no_text() {
        let line: &str = "[12:34.56] [11:22.33]   [43:21:67]      ";
        let verse_text: &str = Lyric::parse_line_text(line);
        assert_eq!(verse_text, "");
    }

    #[test]
    fn wrongly_formatted_verse() {
        let line: &str = "[12:334.56] abcd";
        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);
        assert!(timestamps.is_empty());
        assert_eq!(verse_text, "abcd");

        let line: &str = "[12:34.56 abcd";
        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);
        assert!(timestamps.is_empty());
        assert_eq!(verse_text, "");

        let line: &str = "12:34.56] abcd";
        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);
        assert!(timestamps.is_empty());
        assert_eq!(verse_text, "abcd");
    }

    #[test]
    fn multi_timestamp() {
        let line: &str = "[00:34.88][01:22.33] [10:59.67] This is a verse";
        let timestamps: Vec<usize> = Lyric::parse_line_timestamps(line);
        let verse_text: &str = Lyric::parse_line_text(line);

        assert_eq!(
            timestamps,
            vec![
                34 * 1000 + 880,
                60000 * 1 + 22 * 1000 + 330,
                60000 * 10 + 59 * 1000 + 670
            ]
        );
        assert_eq!(verse_text, "This is a verse");
    }
}

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub enum PrintingStyle {
    FixedTop,
    FixedCenter,
    FixedBottom,
}

pub struct Gui;
struct Terminal;

impl Terminal {
    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Ok(())
    }

    fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    fn set_bold_attribute() -> Result<(), Error> {
        Self::queue_command(SetAttribute(Attribute::Bold))?;
        Ok(())
    }

    fn reset_attributes() -> Result<(), Error> {
        Self::queue_command(SetAttribute(Attribute::Reset))?;
        Ok(())
    }

    fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    fn pool_key_press() -> Result<Option<char>, Error> {
        let key_pressed = match crossterm::event::poll(std::time::Duration::from_millis(100))? {
            true => match crossterm::event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => Some(c),
                    _ => None,
                },
                _ => None,
            },
            false => None,
        };

        Ok(key_pressed)
    }

    fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

impl Gui {
    const TOP_OFFSET: usize = 1;
    const BOT_OFFSET: usize = 1;

    pub fn initialize() -> Result<(), Error> {
        Terminal::initialize()?;
        Terminal::hide_caret()?;
        Terminal::clear_screen()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn pool_keyboard() -> Result<Option<char>, Error> {
        Terminal::pool_key_press()
    }

    pub fn terminate() -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::move_caret_to(Position { col: 0, row: 0 })?;
        Terminal::show_caret()?;
        Terminal::terminate()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Terminal::clear_screen()?;
        Ok(())
    }

    pub fn print_general_error(string: &str) -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::move_caret_to(Position {
            col: 0,
            row: Self::TOP_OFFSET,
        })?;
        Terminal::print(string)?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn print_lyric_not_found_error(artist: &str, title: &str) -> Result<(), Error> {
        let debug = vec![
            "Lyric not found".to_string(),
            "".to_string(),
            format!("Artist: {}", artist),
            format!("Title: {}", title),
        ];

        Terminal::clear_screen()?;
        Gui::print_vector_slice(&debug, 0, 0, debug.len())?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn print_debug(debug_messages: Vec<String>) -> Result<(), Error> {
        let terminal_size = Terminal::size()?;
        let mut pos = Position { col: 0, row: 0 };
        for message in debug_messages {
            pos.col = terminal_size.width - message.len();
            Terminal::move_caret_to(pos)?;
            Terminal::print(&message)?;
            pos.row += 1;
        }
        Ok(())
    }

    pub fn print_vector(vector: &Vec<String>, fixed_index: usize) -> Result<(), Error> {
        let style = Gui::define_printing_style(fixed_index, vector.len())?;
        let terminal_size = Terminal::size()?;
        let printable_size = terminal_size.height - Self::TOP_OFFSET - Self::BOT_OFFSET;
        let start;
        let end;

        if cfg!(debug_assertions) {
            Terminal::clear_screen()?;
        }

        match style {
            PrintingStyle::FixedTop => {
                start = 0;
                end = std::cmp::min(printable_size, vector.len());
                Gui::print_vector_slice(vector, fixed_index, start, end)?;
            }
            PrintingStyle::FixedCenter => {
                start = if (printable_size % 2) == 0 {
                    fixed_index - printable_size / 2
                } else {
                    fixed_index - (printable_size - 1) / 2
                };
                end = start + printable_size;
                Gui::print_vector_slice(vector, fixed_index, start, end)?;
            }
            PrintingStyle::FixedBottom => {
                start = vector.len() - printable_size;
                end = start + printable_size;
                Gui::print_vector_slice(vector, fixed_index, start, end)?;
            }
        };

        /* Debug print BEGIN */
        if cfg!(debug_assertions) {
            let debug = vec![
                "DEBUG".to_string(),
                format!("term height={}", terminal_size.height),
                format!("term width={}", terminal_size.width),
                format!(
                    "printable={}",
                    terminal_size.height - Self::TOP_OFFSET - Self::BOT_OFFSET
                ),
                format!("fixed index={}", fixed_index),
                format!("start={}", start),
                format!("end={}", end),
                format!("vector len={}", vector.len()),
                match style {
                    PrintingStyle::FixedTop => "style=fixed top".to_string(),
                    PrintingStyle::FixedCenter => "style=fixed center".to_string(),
                    PrintingStyle::FixedBottom => "style=fixed bottom".to_string(),
                },
            ];
            Gui::print_debug(debug)?;
        }
        /* Debug print END */

        Terminal::execute()?;
        Ok(())
    }

    fn print_vector_slice(
        vector: &Vec<String>,
        fixed_index: usize,
        start: usize,
        end: usize,
    ) -> Result<(), Error> {
        let term_size = Terminal::size()?;
        let mut cursor = Position {
            col: 0,
            row: Gui::TOP_OFFSET,
        };

        for (index, text) in vector[start..end].iter().enumerate() {
            let t = if text.len() >= term_size.width {
                &text[..term_size.width]
            } else {
                text.as_str()
            };

            cursor.col = if text.len() >= term_size.width {
                0
            } else {
                term_size.width / 2 - text.len() / 2
            };

            Terminal::move_caret_to(cursor)?;
            Terminal::clear_line()?;
            if (start + index) == fixed_index {
                Terminal::set_bold_attribute()?;
                Terminal::print(t)?;
                Terminal::reset_attributes()?;
            } else {
                Terminal::print(t)?;
            }
            cursor.row += 1;
        }

        Ok(())
    }

    fn define_printing_style(
        fixed_index: usize,
        vector_size: usize,
    ) -> Result<PrintingStyle, Error> {
        let terminal_size = Terminal::size()?;
        let printable_size = terminal_size.height - Self::TOP_OFFSET - Self::BOT_OFFSET;

        if vector_size < printable_size {
            return Ok(PrintingStyle::FixedTop);
        }

        let center_row = if (printable_size % 2) == 0 {
            printable_size / 2
        } else {
            (printable_size + 1) / 2
        };

        if fixed_index < center_row {
            return Ok(PrintingStyle::FixedTop);
        }

        if fixed_index > (vector_size - center_row) {
            return Ok(PrintingStyle::FixedBottom);
        }

        Ok(PrintingStyle::FixedCenter)
    }
}

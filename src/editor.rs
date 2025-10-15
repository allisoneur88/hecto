mod terminal;

use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char('h') => self.move_cursor(Direction::Left).unwrap(),
                Char('j') => self.move_cursor(Direction::Down).unwrap(),
                Char('k') => self.move_cursor(Direction::Up).unwrap(),
                Char('l') => self.move_cursor(Direction::Right).unwrap(),
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.cursor_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        // we allow this since we don't care if our welcome message is put eXaCtLy in the middle.
        // is's alllowed to be a bit to the left of right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or down
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn move_cursor(&mut self, direction: Direction) -> Result<(), Error> {
        match direction {
            Direction::Left => {
                if self.cursor_position.x > 0 {
                    self.cursor_position.x = self.cursor_position.x - 1;
                }
            }
            Direction::Down => {
                if self.cursor_position.y < Terminal::size()?.height {
                    self.cursor_position.y = self.cursor_position.y + 1;
                }
            }
            Direction::Up => {
                if self.cursor_position.y > 0 {
                    self.cursor_position.y = self.cursor_position.y - 1;
                }
            }
            Direction::Right => {
                if self.cursor_position.x < Terminal::size()?.width {
                    self.cursor_position.x = self.cursor_position.x + 1;
                }
            }
        }
        Ok(())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

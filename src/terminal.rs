use std::io::Write;

use crossterm::{terminal, event::{Event, self}, ExecutableCommand, cursor};

pub struct TerminalController {
    stdout: std::io::Stdout,
}

impl TerminalController {
    pub fn init() -> Result<Self, std::io::Error> {
        // put terminal in raw mode 
        let mut stdout = std::io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        Ok(TerminalController { stdout: std::io::stdout() })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).map(|_| ())
    }

    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::Show).map(|_| ())
    }

    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::Hide).map(|_| ())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdout.flush()
    }

    pub fn handle_keypress(&self) -> Result<Key, std::io::Error> {
        log::info!("handle keypress");
        let event = event::read()?;
        match event {
            Event::Key(key_event) => Ok(Key::map_code(key_event.code)),
            _ => Ok(Key::Other),
        }
    }

    pub fn clean(&mut self) -> Result<(), std::io::Error> {
        self.clear()?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        log::info!("cleaning");
        Ok(())
    }
}

pub enum Key {
    Char(char),
    Esc,
    Other,
}

impl Key {
    fn map_code(code: crossterm::event::KeyCode) -> Self {
        match code {
            event::KeyCode::Char(c) => Key::Char(c),
            event::KeyCode::Esc => Key::Esc,
            _ => Key::Other,
        }
    }
}
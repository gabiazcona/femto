use std::io::Write;
use crate::editor::Position;

use crossterm::{terminal, event::{Event, self}, ExecutableCommand, cursor};

pub struct TerminalController {
    size: Size,
    stdout: std::io::Stdout,
}

pub struct Size { 
    height: u16, 
    width: u16 
}


impl TerminalController {
    pub fn init() -> Result<Self, std::io::Error> {
        // put terminal in raw mode 
        let mut stdout = std::io::stdout();
        let (cols, rows) = terminal::size()?;
        log::info!("terminal size: cols {}, rows {}", cols, rows);
        stdout.execute(terminal::EnterAlternateScreen)?.execute(terminal::SetSize(cols, rows))?;
        terminal::enable_raw_mode()?;
        Ok(TerminalController { size: Size { height: cols, width: rows}, stdout })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).map(|_| ())
    }

    pub fn clear_line(&mut self) {
        if let Err(e) = self.stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine)) {
            log::error!("Error: {:?}\r", e);
        }
    }

    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::Show).map(|_| ())
    }

    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::Hide).map(|_| ())
    }

    pub fn get_cursor_position(&mut self) -> Result<Position, std::io::Error> {
        cursor::position().map(|(x, y)| Position::new(x as usize, y as usize))
    }

    pub fn position_cursor(&mut self, x: u16, y: u16) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::MoveTo(x, y)).map(|_| ())
    }

    pub fn move_down_cursor(&mut self) {
        if let Err(e) = self.stdout.execute(cursor::MoveDown(1)) { 
            log::error!("Error: {:?}\r", e); 
        };
    }

    pub fn move_up_cursor(&mut self) {
        if let Err(e) = self.stdout.execute(cursor::MoveUp(1)) { 
            log::error!("Error: {:?}\r", e); 
        };
    }

    pub fn move_left_cursor(&mut self) {
        if let Err(e) = self.stdout.execute(cursor::MoveLeft(1)) { 
            log::error!("Error: {:?}\r", e); 
        };
    }

    pub fn move_right_cursor(&mut self) {
        if let Err(e) = self.stdout.execute(cursor::MoveRight(1)) { 
            log::error!("Error: {:?}\r", e); 
        };
    }
    
    pub fn save_cursor_position(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::SavePosition).map(|_| ())
    }

    pub fn restore_cursor_position(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::RestorePosition).map(|_| ())
    }

    pub fn enable_blinking_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::EnableBlinking).map(|_| ())
    }

    pub fn disable_blinking_cursor(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::DisableBlinking).map(|_| ())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdout.flush()
    }

    pub fn handle_keypress(&self) -> Result<Key, std::io::Error> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) => {
                //log::debug!("handle keypress: {}", key_event.code);
                Ok(Key::map_code(key_event.code))
            },
            _ => Ok(Key::Other),
        }
    }

    pub fn clean(&mut self) -> Result<(), std::io::Error> {
        self.clear()?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        log::info!("clean terminal");
        Ok(())
    }

    pub fn get_height(&self) -> u16 {
        self.size.height
    }
    pub fn get_width(&self) -> u16 {
        self.size.width
    }
}

pub enum Key {
    Char(char),
    Esc,
    Down,
    Up,
    Left,
    Right,
    Other,
}

impl Key {
    fn map_code(code: crossterm::event::KeyCode) -> Self {
        match code {
            event::KeyCode::Char(c) => Key::Char(c),
            event::KeyCode::Esc => Key::Esc,
            event::KeyCode::Down => Key::Down,
            event::KeyCode::Up => Key::Up,
            event::KeyCode::Left => Key::Left,
            event::KeyCode::Right => Key::Right,
            _ => Key::Other,
        }
    }
}
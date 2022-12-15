use crossterm::{terminal, event::{Event, self}, ExecutableCommand};

pub struct TerminalController {
    stdout: std::io::Stdout,
}

impl TerminalController {
    pub fn init() -> Result<Self, std::io::Error> {
        // put terminal in raw mode 
        crossterm::terminal::enable_raw_mode()?;
        Ok(TerminalController { stdout: std::io::stdout() })
    }

    pub fn clear(&mut self) {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All));
    }

    pub fn show_cursor() {
        todo!()
    }
    pub fn hide_cursor() {
        todo!()
    }

    pub fn handle_keypress(&self) -> Result<Key, std::io::Error> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) => Ok(Key::map_code(key_event.code)),
            _ => Ok(Key::Other),
        }
    }

    pub fn clean(&mut self) -> Result<(), std::io::Error> {
        self.clear();
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
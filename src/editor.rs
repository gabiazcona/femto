use crate::{document::Document, terminal::{TerminalController, Key}};


pub struct Editor {
    document: Document,
    mode: Mode,
}

impl Editor {
    pub fn init(filename: &str) -> Result<Self, std::io::Error> {
        Ok(Editor {
            document: Document::open(filename)?,
            mode: Mode::Control,
        })
    }

    pub fn start(&mut self) {
        println!("FEMTO\n\n");
        loop {
            self.render_document();
        }
    }

    pub fn process_key(&mut self, key: &Key) -> Status {
        match self.mode {
            Mode::Control => self.process_key_control(key),
            Mode::Edit => self.process_key_edit(key),
        }
    }

    fn process_key_edit(&mut self, key: &Key) -> Status {
        match key {
            Key::Char(c) => println!("{}", c),
            Key::Esc => self.mode = Mode::Control,
            _ => {},
        }
        Status::Continue
    }

    fn process_key_control(&mut self, key: &Key) -> Status {
        match key {
            Key::Char('q') => Status::Quit,
            Key::Char('e') => {
                self.mode = Mode::Edit;
                Status::Continue
            }
            // Key::Alt(c) => println!("^{}", c),
            // Key::Ctrl(c) => println!("*{}", c),
            // Key::Esc => println!("ESC"),
            // Key::Left => println!("flechita"),
            // Key::Right => println!("→"),
            // Key::Up => println!("↑"),
            // Key::Down => println!("↓"),
            // Key::Backspace => println!("×"),
            _ => Status::Continue
        }
    }

    fn render_document(&mut self) {
        println!("{}", self.document.filename);
        println!("{}", self.document.contents);
        TerminalController::show_cursor();
    }
}

pub enum Status {
    Quit,
    Continue,
}

enum Mode {
    Edit,
    Control,
}
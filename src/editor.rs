use crate::{document::Document, terminal::{TerminalController, Key}};


pub struct Editor {
    document: Document,
    mode: Mode,
}

impl Editor {
    pub fn init(filename: Option<&String>) -> Result<Self, std::io::Error> {
        
        Ok(Editor {
            document: Document::open(filename)?,
            mode: Mode::Control,
        })
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

    // fn refresh_screen(&self) -> Result<(), std::io::Error> {
    //     TerminalController::hide_cursor();
    //     TerminalController::cursor_position(&Position::default());
    //     if self.should_quit {
    //         TerminalController::clear_screen();
    //         println!("Bye!\r");
    //     } else {
    //         self.draw_rows();
    //         TerminalController::cursor_position(&Position {
    //             x: self.cursor_position.x.saturating_sub(self.offset.x),
    //             y: self.cursor_position.y.saturating_sub(self.offset.y),
    //         });
    //     }
    //     Terminal::cursor_show();
    //     Terminal::flush()
    // }

    pub fn render_document(&mut self, terminal: &mut TerminalController) {
        terminal.hide_cursor();

        println!("{}", self.document.contents);
        terminal.show_cursor();
        terminal.flush();
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
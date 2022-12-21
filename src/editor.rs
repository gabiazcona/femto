use crate::{document::Document, terminal::{TerminalController, Key}};


pub struct Editor {
    document: Document,
    mode: Mode,
    offset: Position,
}


impl Editor {
    pub fn init(filename: Option<&String>) -> Result<Self, std::io::Error> {
        
        Ok(Editor {
            document: Document::open(filename)?,
            mode: Mode::Control,
            offset: Position{x: 0, y: 0},
        })
    }


    pub fn process_key(&mut self, key: &Key, interface_controller: &mut TerminalController) -> Status {
        match self.mode {
            Mode::Control => self.process_key_control(key, interface_controller),
            Mode::Edit => self.process_key_edit(key, interface_controller),
        }
    }

    fn process_key_edit(&mut self, key: &Key, interface_controller: &mut TerminalController) -> Status {
        match key {
            Key::Char(c) => {
                let cursor_position = interface_controller.get_cursor_position().unwrap_or(Position::default());
                self.document.edit_rows(c.clone(), &cursor_position);
                interface_controller.move_right_cursor();
            },
            Key::Esc => self.mode = Mode::Control,
            _ => {},
        };
        Status::Continue
    }

    fn process_key_control(&mut self, key: &Key, interface_controller: &mut TerminalController) -> Status {
        match key {
            Key::Char('q') => Status::Quit,
            Key::Char('e') => {
                self.mode = Mode::Edit;
                Status::Continue
            },
            Key::Char('x') => {
                self.document.save_file();
                Status::Continue
            },
            Key::Down => {
                interface_controller.move_down_cursor();
                Status::Continue
            },
            Key::Up => {
                interface_controller.move_up_cursor();
                Status::Continue
            },
            Key::Left => {
                interface_controller.move_left_cursor();
                Status::Continue
            },
            Key::Right => {
                interface_controller.move_right_cursor();
                Status::Continue
            },
            _ => Status::Continue
        }
    }

    pub fn render_document(&mut self, terminal: &mut TerminalController) -> Result<(), std::io::Error> {
        terminal.save_cursor_position()?;
        terminal.hide_cursor()?;

        terminal.position_cursor(0, 0)?;
        self.document.render(self.offset.x, self.offset.y, terminal);
        terminal.restore_cursor_position()?;
        match self.mode {
            Mode::Edit => terminal.enable_blinking_cursor()?,
            Mode::Control => terminal.disable_blinking_cursor()?,
        };
        terminal.show_cursor()?;
        terminal.flush()?;
        Ok(())
    }
}


#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
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
use crate::{document::Document, terminal::TerminalController};


pub struct Editor {
    document: Document,
}

impl Editor {
    pub fn init(filename: &str) -> Result<Self, std::io::Error> {
        Ok(Editor {
            document: Document::open(filename)?,
        })
    }

    pub fn start(&mut self) {
        TerminalController::clear();
        println!("FEMTO\n\n");
        self.render_document();
    }

    fn render_document(&mut self) {
        println!("{}", self.document.filename);
        println!("{}", self.document.contents);
    }
}
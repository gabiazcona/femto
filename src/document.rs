use std::{fs, iter::FlatMap};

use crate::{terminal, editor::Position};

#[derive(Default)]
pub struct Document {
    pub filename: Option<String>,
    lines: Vec<Line>,
}

impl Document {
    pub fn open(filename: Option<&String>) -> Result<Document, std::io::Error>  {
        let mut lines = Vec::new();
        match filename {
            Some(filename) => {
                log::info!("Opening and reading file: {}", filename);
                let contents = fs::read_to_string(filename)?;
                for value in contents.lines() {
                    lines.push(Line::new(value));
                }
                Ok( Document { filename: Some(filename.clone()), lines } )
            },
                None => Ok(Document { filename: None, lines }),
        }
    }

    pub fn save_file(&self) {
        todo!()
    }

    pub fn render(&self, offset_x: usize, offset_y: usize, terminal: &mut terminal::TerminalController) {
        let section: Vec<String> = self.get_section(offset_x, offset_y, terminal.get_height().into(), terminal.get_width().into());
        for line in section {
            print!("{}\r", line)
        }
    }

    fn get_section(&self, offset_x: usize, offset_y: usize, width: usize, height: usize) -> Vec<String> {
        let mut section: Vec<String> = Vec::new();
        for line in self.lines.iter() {
            let start = std::cmp::min(offset_x, line.length);
            let end = std::cmp::min(line.length, offset_x + width);
            let subline = &line.contents[start..end];
            section.push(subline.into())
        };
        section
    }


    pub fn edit_rows(&mut self, c: char, position: &Position) {
        self.lines.get_mut(position.y).map(|row| row.write(c, position.x));
    }
    
}

struct Line {
    contents: String,
    length: usize,
}

impl Line {
    fn new(content: &str) -> Line {
        let contents = String::from(content);
        Line {
            contents: contents.clone(),
            length: contents.len() as usize,
        }
    }

    fn get_contents(&self, start: usize, end: usize) -> &str {
        &self.contents[start..end]
    }

    fn write(&mut self, c: char, position: usize) {
        self.contents.insert(position, c);
        self.length += 1;
    }
}
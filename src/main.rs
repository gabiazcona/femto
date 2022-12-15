use std::{env, fs};
use editor::{Editor, Status};
use terminal::TerminalController;
use log;
use simplelog::*;

mod document;
mod terminal;
mod editor;

fn main() {    
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]  
    } else {
        panic!("You need to provide a filename");
    };

    WriteLogger::init(LevelFilter::Info, Config::default(), fs::File::create("femto.log").unwrap()).unwrap();
    log::info!("Starting FEMTO");

    let mut femto = App::init(filename).expect(&format!("Cannot open file: {}", filename));
    femto.run();
}

struct App {
    terminal: TerminalController,
    editor: Editor,
}

impl App {
    fn init(filename: &str) -> Result<Self, std::io::Error> {
        Ok(App {
            terminal: TerminalController::init()?,
            editor: Editor::init(filename)?,
        })
    }
    fn run(&mut self) {
        loop {
            if let Ok(key) = self.terminal.handle_keypress() {
                match self.editor.process_key(&key) {
                    Status::Quit =>  break,
                    Status::Continue => {}
                }
            }
        }
        if let Err(e) = self.terminal.clean() {
            println!("Error: {:?}\r", e);
        };
    }
}

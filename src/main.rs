use std::{env, fs};
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

    let mut femto = editor::Editor::init(filename).expect(&format!("Cannot open file: {}", filename));
    femto.start();
}

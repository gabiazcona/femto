use std::fs;

pub struct Document {
    pub filename: String,
    pub contents: String,
}

impl Document {
    pub fn open(filename: &str) -> Result<Document, std::io::Error>  {
        log::info!("Opening and reading file: {}\n", filename);

        let contents = fs::read_to_string(filename)?;
        Ok( Document { filename: filename.to_string(), contents } )
    }

}
use std::fs;

#[derive(Default)]
pub struct Document {
    pub filename: Option<String>,
    pub contents: String,
}

impl Document {
    pub fn open(filename: Option<&String>) -> Result<Document, std::io::Error>  {
        match filename {
            Some(filename) => {
                log::info!("Opening and reading file: {}\n", filename);
                let contents = fs::read_to_string(filename)?;
                Ok( Document { filename: Some(filename.clone()), contents } )
            },
                None => Ok(Self::default()),
        }
    }
}
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

pub struct FileResource {
    pub path: String,
    pub content: String
}

impl FileResource {
    pub fn write_file(&self) -> Result<(), Error> {
        let file_handle = File::create(&self.path);
        let bytes = self.content.to_string().into_bytes();
        match file_handle {
            Ok(mut handle) => {
                return handle.write_all(&bytes[..]);
            },
            Err(err) => return Err(err)
        }
    }

    pub fn is_valid(&self) -> bool {
        self.path.len() > 0 && self.content.len() > 0
    }

    pub fn error_messages(&self) -> Vec<String> {
        let mut error_messages = Vec::new();

        if self.path.len() == 0 {
            error_messages.push("File: path is missing".into());
        }

        if self.content.len() == 0 {
            error_messages.push("File: content is missing".into());
        }

        error_messages
    }
}

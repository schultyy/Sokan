extern crate crypto;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub struct FileResource {
    pub path: String,
    pub content: String
}

impl FileResource {
    pub fn create_from_path(path: &String) -> Result<FileResource, Error> {
        let file_handle = File::open(path);
        match file_handle {
            Ok(mut handle) => {
                let mut contents = String::new();
                match handle.read_to_string(&mut contents) {
                    Ok(_) => return Ok(FileResource {
                        path: path.to_string(),
                        content: contents.to_string()
                    }),
                    Err(err) => return Err(err)
                }
            },
            Err(err) => return Err(err)
        }
    }

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
        self.error_messages().len() == 0
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

    pub fn hash(&self) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&self.content);
        sha.result_str()
    }
}

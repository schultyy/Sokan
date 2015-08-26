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
}

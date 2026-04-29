use std::fs::{File, Metadata};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
pub mod print_input;
pub mod get_input;
pub enum Input {
    Stdin(()),
    File(PathBuf),
}

impl Input {
    pub fn open_file(&self) -> io::Result<File> {
        match self {
            Input::Stdin(_) => {
                Err(Error::from(ErrorKind::InvalidInput))
            }
            Input::File(f) => File::open(f),
        }
    }

    pub fn metadata(&self) -> io::Result<Metadata> {
        match self {
            Input::Stdin(_) => {
                Err(Error::from(ErrorKind::InvalidInput))
            }
            Input::File(f) => f.metadata(),
        }
    }
}

impl Clone for Input {
    fn clone(&self) -> Input {
        match self {
            Input::Stdin(_) => Input::Stdin(()),
            Input::File(f) => Input::File(f.clone()),
        }
    }
}

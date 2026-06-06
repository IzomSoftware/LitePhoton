use std::fs::{File, Metadata};
use std::io::{self, BufReader, Read};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use memmap2::Mmap;

pub trait Input {
    fn open(&self) -> io::Result<File> {
        Err(Error::from(ErrorKind::InvalidInput))
    }
    fn get_metadata(&self) -> io::Result<Metadata> {
        Err(Error::from(ErrorKind::InvalidInput))
    }
    fn mmap(&self) -> io::Result<Mmap> {
        Err(Error::from(ErrorKind::InvalidInput))
    }
    fn create_read_buf(&self) -> io::Result<BufReader<Box<dyn Read + Send>>>{
        Err(Error::from(ErrorKind::InvalidInput))
    }
}

pub enum InputType {
    Stdin,
    File(PathBuf),
}
pub struct StdinInput;

impl Input for StdinInput {
    fn create_read_buf(&self) -> io::Result<BufReader<Box<dyn Read + Send>>> {
        Ok(BufReader::with_capacity(64 * 1024,Box::new(io::stdin())))
    }
}

pub struct FileInput {
    path: PathBuf,
}

impl Input for FileInput {
    fn open(&self) -> io::Result<File> {
        File::open(&self.path)
    }
    fn get_metadata(&self) -> io::Result<Metadata> {
        self.path.metadata()
    }
    fn mmap(&self) -> io::Result<Mmap> {
        unsafe {memmap2::Mmap::map(&self.open()?)}
    }
}

pub struct InputBuilder;

impl InputBuilder {
    pub fn new(input_type: InputType) -> Box<dyn Input> {
        match input_type {
            InputType::File(path) => Box::new(FileInput { path }),
            InputType::Stdin => Box::new(StdinInput),
        }
    }
}
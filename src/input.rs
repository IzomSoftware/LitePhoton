use std::fs::{File, Metadata};
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use memmap2::Mmap;

pub trait Input {
    fn open(&self) -> io::Result<File> {
        unimplemented!()
    }
    fn get_metadata(&self) -> io::Result<Metadata> {
        unimplemented!()
    }
    fn mmap(&self) -> io::Result<Mmap> {
        unimplemented!()
    }
    fn create_read_buf(&self) -> io::Result<BufReader<Box<dyn Read + Send>>>{
        unimplemented!()
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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(input_type: InputType) -> Box<dyn Input + Sync> {
        match input_type {
            InputType::File(path) => Box::new(FileInput { path }),
            InputType::Stdin => Box::new(StdinInput),
        }
    }
}
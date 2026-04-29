use crate::input::Input;
use memmap2::Mmap;
use regex::bytes::Regex;
use std::{
    error,
    io::{BufReader, BufWriter, Read, Write},
};
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadInputError {
    #[error("Could map stdin.")]
    CannotMapStdin,
    #[error("The file is empty.")]
    EmptyFile,
}

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Method {
    Simple,
    Rayon,
    StdThread,
}

pub fn create_buf_write<W>(inner: W) -> BufWriter<W>
where
    W: Sized + Write,
{
    BufWriter::with_capacity(64 * 1024, inner)
}
pub fn create_read_buf<R>(inner: R) -> BufReader<R>
where
    R: Sized + Read,
{
    BufReader::with_capacity(64 * 1024, inner)
}
// pub fn create_line_buf() -> Vec<u8> {
//     Vec::with_capacity(4 * 1024)
// }

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn map_file(input: Input) -> std::io::Result<Mmap> {
    unsafe { Mmap::map(&input.open_file()?) }
}
pub fn write_all<W>(writer: &mut BufWriter<W>, line: &[u8]) -> std::io::Result<()>
where
    W: Sized + Write,
{
    writer.write_all(line)
}
pub fn flush<W>(writer: &mut BufWriter<W>) -> std::io::Result<()>
where
    W: Sized + Write,
{
    writer.flush()
}
pub fn check_line(
    line: &[u8],
    keyword: &[u8],
    regex: &[u8],
) -> Result<bool, Box<dyn error::Error>> {
    let regex_result = if regex.is_empty() {
        true
    } else {
        let new_regex = Regex::new(String::from_utf8_lossy(regex).into_owned().as_str())?;
        Regex::is_match(&new_regex, line)
    };

    Ok((keyword.is_empty() || memchr::memmem::find(line, keyword).is_some()) && regex_result)
}

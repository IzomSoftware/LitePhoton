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
pub fn check_keyword<'a>(line: &'a [u8], keyword: &'a [u8]) -> Option<&'a [u8]> {
    if !keyword.is_empty() && memchr::memmem::find(line, keyword).is_some() {
        return Some(line);
    }
    None
}
pub fn check_regex<'a>(
    line: &'a [u8],
    regex_bytes: &'a [u8],
) -> Result<Option<Vec<&'a [u8]>>, Box<dyn error::Error>> {
    if !regex_bytes.is_empty() {
        let mut results = vec![];
        let regex = &Regex::new(String::from_utf8_lossy(regex_bytes).into_owned().as_str())?;

        for matched in regex.find_iter(line) {
            results.push(matched.as_bytes());
        }

        return Ok(Some(results));
    }

    Ok(None)
}

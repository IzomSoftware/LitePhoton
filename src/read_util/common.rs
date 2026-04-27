use crate::input::Input;
use memmap2::Mmap;
use thiserror::Error;
use std::io::{BufWriter, Write};
use strum_macros::EnumString;

#[derive(Debug, Error)]
pub enum ReadInputError{
    #[error("Could not use chunk mode while input is STDIN.")]
    CannotUseStdinWithChunkMode,
    #[error("The file is empty.")]
    EmptyFile,
}

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    Normal,
    Chunk,
}

pub unsafe fn map_file(input: Input) -> std::io::Result<Mmap> {
    unsafe { Mmap::map(&input.open_file()?) }
}

pub fn write_all<W>(writer: &mut BufWriter<W>, line: &[u8]) -> std::io::Result<()>
where
    W: Sized + Write,
{
    Ok(writer.write_all(line)?)
}
pub fn flush<W>(writer: &mut BufWriter<W>) -> std::io::Result<()>
where
    W: Sized + Write,
{
    Ok(writer.flush()?)
}
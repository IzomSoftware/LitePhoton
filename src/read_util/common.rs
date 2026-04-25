use crate::input::Input;
use memmap2::Mmap;
use std::io::{BufWriter, Write};

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

pub fn fail<W>(writer: &mut BufWriter<W>, line: &[u8]) -> std::io::Result<()>
where
    W: Sized + Write,
{
    write_all(writer, line)?;
    flush(writer)?;
    Ok(())
}

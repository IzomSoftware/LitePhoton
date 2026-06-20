use std::io::{BufWriter, Write};

pub fn create_buf_write<W>(inner: W) -> BufWriter<W>
where
    W: Sized + Write,
{
    BufWriter::with_capacity(64 * 1024, inner)
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

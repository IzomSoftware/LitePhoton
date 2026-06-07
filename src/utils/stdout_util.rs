use std::io::{self, BufWriter, Stdout, Write};

use crate::utils;

pub trait BufWriterImpl {
    fn write_all_with_newline(&mut self, bytes: &[u8]) -> io::Result<()> ;
}
impl<W> BufWriterImpl for BufWriter<W>
where
    W: Write + Send,
{
    fn write_all_with_newline(&mut self, buf: &[u8]) -> io::Result<()> {
        if let Err(err) = self.write_all(buf) {
            return Err(err);
        }
        if let Err(err) = self.write_all(b"\n") {
            return Err(err);
        }
        Ok(())
    }
}

pub fn create_stdout_buf_write() -> BufWriter<Stdout> {
    utils::write_util::create_buf_write(io::stdout())
}

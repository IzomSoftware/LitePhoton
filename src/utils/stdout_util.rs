use std::io::{self, BufWriter, Stdout};

use crate::utils;

pub fn create_stdout_buf_write() -> BufWriter<Stdout> {
    utils::write_util::create_buf_write(io::stdout())
}
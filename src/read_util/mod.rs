pub mod common;

use crate::input::Input;
use crate::read_util::common::{Mode, ReadInputError, flush, map_file, write_all};
use std::error;
use std::io::{BufReader, BufWriter, Read, stdin, stdout};
use std::sync::Arc;

pub fn read_stdin(keyword: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout());
    let mut reader = BufReader::with_capacity(64 * 1024, stdin());
    let mut line_buff = Vec::with_capacity(4 * 1024);
    let mut read_buff = [0u8; 256 * 1024];
    let mut begin = 0usize;
    let mut i = 0usize;

    loop {
        match reader.read(&mut read_buff) {
            Ok(0) => {
                if !line_buff.is_empty() {
                    let line = &line_buff[..];

                    if keyword.is_empty() || memchr::memmem::find(line, keyword).is_some() {
                        write_all(&mut writer, line)?;
                    }

                    write_all(&mut writer, b"\n")?;
                    // fail(&mut writer, b"\n")?;
                }

                flush(&mut writer)?;
                // fail(&mut writer, b"")?;
                break;
            }
            Ok(size) => {
                line_buff.extend_from_slice(&read_buff[..size]);

                while i < line_buff.len() {
                    if line_buff[i] == b'\n' {
                        let line = &line_buff[begin..=i];

                        if keyword.is_empty() || memchr::memmem::find(line, keyword).is_some() {
                            write_all(&mut writer, line)?;
                        }

                        begin = i + 1;
                    }
                    i += 1;
                }

                if begin == 0 {
                    continue;
                }

                if begin < line_buff.len() {
                    line_buff.drain(0..begin);
                } else {
                    line_buff.clear();
                }
            }
            Err(_) => {
                flush(&mut writer)?;
                break;
            }
        }
    }

    Ok(())
}

pub unsafe fn read_file(keyword: &[u8], input: Input) -> Result<(), Box<dyn error::Error>> {
    let mmap = unsafe { map_file(input)? };
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout());
    let mut begin = 0usize;
    let mut i = 0usize;

    while i <= mmap.len() {
        match memchr::memchr(b'\n', &mmap[i..]) {
            Some(0) => {
                break;
            }
            Some(pos) => {
                let end = i + pos;
                let line = &mmap[begin..=end];

                if keyword.is_empty() || memchr::memmem::find(line, keyword).is_some() {
                    write_all(&mut writer, line)?;
                }

                begin = end + 1;
                i = begin;
            }
            None => {
                if begin < mmap.len() {
                    let line = &mmap[begin..];

                    if keyword.is_empty() || memchr::memmem::find(line, keyword).is_some() {
                        write_all(&mut writer, line)?;
                        write_all(&mut writer, b"\n")?;
                    }
                }

                flush(&mut writer)?;
                break;
            }
        }
    }

    Ok(())
}

pub unsafe fn read_file_chunk(
    keyword: &[u8],
    input: Input,
    num_workers: usize,
) -> Result<(), Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotUseStdinWithChunkMode.into());
        }
    };
    let file_size = input.metadata()?.len();
    if file_size == 0 {
        return Err(ReadInputError::EmptyFile.into());
    }
    let mmap = unsafe { map_file(input)? };
    let mmap = Arc::new(mmap);
    let keyword: Arc<&[u8]> = Arc::new(keyword);

    rayon::scope(move |scope| {
        for id in 0..num_workers {
            let keyword = keyword.clone();
            let mmap = mmap.clone();
            let mmap_size = mmap.len();
            let chunk_size = mmap_size / num_workers;
            let mut writer = BufWriter::with_capacity(64 * 1024, stdout());
            let mut begin = id * chunk_size;
            let mut end = begin + chunk_size;

            scope.spawn(move |_| {
                let mmap = &mmap[..];

                if id > 0 {
                    if let Some(b) = memchr::memchr(b'\n', &mmap[begin..]) {
                        begin += b + 1;
                    } else {
                        return;
                    }
                }

                if end < mmap_size {
                    if let Some(b) = memchr::memchr(b'\n', &mmap[end..]) {
                        end += b + 1;
                    } else {
                        end = mmap_size;
                    }
                }

                while begin < end {
                    match memchr::memchr(b'\n', &mmap[begin..end]) {
                        Some(0) => {
                            break;
                        }
                        Some(size) => {
                            let end = begin + size + 1;
                            let line = &mmap[begin..end];

                            if keyword.is_empty() || memchr::memmem::find(line, &keyword).is_some()
                            {
                                write_all(&mut writer, line).expect("");
                            }

                            begin = end;
                        }
                        None => {
                            let line = &mmap[begin..end];

                            if !line.is_empty()
                                && (keyword.is_empty()
                                    || memchr::memmem::find(line, &keyword).is_some())
                            {
                                write_all(&mut writer, line).expect("");
                                write_all(&mut writer, b"\n").expect("");
                            }

                            break;
                        }
                    }
                }

                flush(&mut writer).expect("");
            });
        }
    });

    Ok(())
}

pub unsafe fn read_input(
    mode: Mode,
    input: Input,
    _stable: bool,
    keyword: String,
) -> Result<(), Box<dyn error::Error>> {
    let keyword = keyword.as_bytes();

    match input {
        // Use BufReader with stdin
        Input::Stdin(_) => read_stdin(keyword),
        // Use MemMap2 with with the file
        Input::File(_) => match mode {
            Mode::Normal => unsafe { read_file(keyword, input) },
            Mode::Chunk => unsafe { read_file_chunk(keyword, input, rayon::current_num_threads()) },
        },
    }
}

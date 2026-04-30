use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::common::{
    Method, ReadInputError, check_keyword, check_regex, create_buf_write, create_read_buf, flush,
    map_file, write_all,
};
use crate::input::Input;
use std::io::{Read, Write};
use std::sync::Arc;
use std::{error, io, thread};

pub fn stdin_normal(keyword: &[u8], regex: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let mut writer = create_buf_write(io::stdout());
    let mut reader = create_read_buf(io::stdin());
    let mut line_buff = Vec::with_capacity(4 * 1024);
    let mut read_buff = [0u8; 256 * 1024];
    let mut begin = 0usize;
    let mut i = 0usize;

    loop {
        match reader.read(&mut read_buff) {
            Ok(0) => {
                if !line_buff.is_empty() {
                    let line = &line_buff[..];

                    if let Some(result) = check_keyword(line, keyword) {
                        write_all(&mut writer, result)?;
                        write_all(&mut writer, b"\n")?;
                    }

                    if let Some(results) = check_regex(line, regex)? {
                        for result in results {
                            write_all(&mut writer, result)?;
                            write_all(&mut writer, b"\n")?;
                        }
                    }
                }

                flush(&mut writer)?;
                break;
            }
            Ok(size) => {
                line_buff.extend_from_slice(&read_buff[..size]);

                while i < line_buff.len() {
                    if line_buff[i] == b'\n' {
                        let line = &line_buff[begin..=i];

                        if let Some(result) = check_keyword(line, keyword) {
                            write_all(&mut writer, result)?;
                        }

                        if let Some(results) = check_regex(line, regex)? {
                            for result in results {
                                write_all(&mut writer, result)?;
                            }
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

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn file_normal(
    keyword: &[u8],
    regex: &[u8],
    input: Input,
) -> Result<(), Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let mmap = unsafe { map_file(input)? };
    let mut writer = create_buf_write(io::stdout());
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

                if let Some(result) = check_keyword(line, keyword) {
                    write_all(&mut writer, result)?;
                }

                if let Some(results) = check_regex(line, regex)? {
                    for result in results {
                        write_all(&mut writer, result)?;
                        write_all(&mut writer, b"\n")?;
                    }
                }

                begin = end + 1;
                i = begin;
            }
            None => {
                if begin < mmap.len() {
                    let line = &mmap[begin..];

                    if let Some(result) = check_keyword(line, keyword) {
                        write_all(&mut writer, result)?;
                        write_all(&mut writer, b"\n")?;
                    }

                    if let Some(results) = check_regex(line, regex)? {
                        for result in results {
                            write_all(&mut writer, result)?;
                            write_all(&mut writer, b"\n")?;
                        }
                    }
                }

                flush(&mut writer)?;
                break;
            }
        }
    }

    Ok(())
}

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn file_chunk_rayon(
    keyword: &[u8],
    regex: &[u8],
    input: Input,
) -> Result<(), Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let file_size = input.metadata()?.len();
    if file_size == 0 {
        return Err(ReadInputError::EmptyFile.into());
    }
    let mmap = unsafe { map_file(input)? };
    let mmap = &mmap[..];

    mmap.split(|&b| b == b'\n')
        .par_bridge()
        .filter_map(|line| {
            if let Some(result) = check_keyword(line, keyword) {
                return Some(vec![result]);
            }

            if let Ok(Some(results)) = check_regex(line, regex) {
                return Some(results);
            }

            None
        })
        .for_each(|lines| {
            let mut writer = create_buf_write(io::stdout());

            for line in lines {
                writer
                    .write_all(line)
                    .expect("input/print_input.rs: Couldn't write to stdout");
                writer
                    .write_all(b"\n")
                    .expect("input/print_input.rs: Couldn't write to stdout");
            }
        });

    Ok(())
}

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn file_chunk_std(
    keyword: &[u8],
    regex: &[u8],
    input: Input,
    num_workers: usize,
) -> Result<(), Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let file_size = input.metadata()?.len();
    if file_size == 0 {
        return Err(ReadInputError::EmptyFile.into());
    }
    let mmap = unsafe { map_file(input)? };
    let mmap = Arc::new(mmap);
    let keyword: Arc<&[u8]> = Arc::new(keyword);
    let regex: Arc<&[u8]> = Arc::new(regex);

    thread::scope(move |scope| {
        for id in 0..num_workers {
            let keyword = keyword.clone();
            let regex = regex.clone();
            let mmap = mmap.clone();
            let mmap_size = mmap.len();
            let chunk_size = mmap_size / num_workers;
            let mut writer = create_buf_write(io::stdout());
            let mut begin = id * chunk_size;
            let mut end = begin + chunk_size;

            scope.spawn(move || {
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
                            let end = begin + size;
                            let line = &mmap[begin..=end];

                            if let Some(result) = check_keyword(line, &keyword) {
                                write_all(&mut writer, result).expect("");
                            }

                            if let Ok(Some(results)) = check_regex(line, &regex) {
                                for result in results {
                                    write_all(&mut writer, result).expect("");
                                }
                            }

                            begin = end;
                        }
                        None => {
                            let line = &mmap[begin..end];

                            if !line.is_empty()
                                && let Some(result) = check_keyword(line, &keyword)
                            {
                                write_all(&mut writer, result).expect("");
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

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn input(
    method: Method,
    input: Input,
    #[allow(unused)] stable: bool,
    keyword: String,
    regex: String,
) -> Result<(), Box<dyn error::Error>> {
    let keyword = keyword.as_bytes();
    let regex = regex.as_bytes();

    match input {
        // Use BufReader with stdin
        Input::Stdin(_) => stdin_normal(keyword, regex),
        // Use MemMap2 with with files
        Input::File(_) => match method {
            Method::Simple =>
            // # Safety
            // Memory mapping a file is not a fail safe thing to do
            unsafe { file_normal(keyword, regex, input) },
            Method::Rayon =>
            // # Safety
            // Memory mapping a file is not a fail safe thing to do
            unsafe { file_chunk_rayon(keyword, regex, input) },
            Method::StdThread =>
            // # Safety
            // Memory mapping a file is not a fail safe thing to do
            unsafe { file_chunk_std(keyword, regex, input, rayon::current_num_threads()) },
        },
    }
}

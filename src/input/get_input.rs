use crate::common::{
    Method, ReadInputError, check_keyword, check_regex, create_read_buf, map_file,
};
use crate::input::Input;
use std::collections::HashSet;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{error, io, thread};

pub fn stdin_normal(keyword: &[u8], regex: &[u8]) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut results: Vec<String> = vec![];
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

                    if keyword.is_empty() && regex.is_empty() {
                        results.push(String::from_utf8_lossy(line).into());
                    }

                    if !keyword.is_empty() {
                        if let Some(result) = check_keyword(line, keyword) {
                            results.push(String::from_utf8_lossy(result).into());
                        }
                    }

                    if !regex.is_empty() {
                        if let Some(regex_results) = check_regex(line, regex)? {
                            for result in regex_results {
                                results.push(String::from_utf8_lossy(result).into());
                            }
                        }
                    }
                }
                break;
            }
            Ok(size) => {
                line_buff.extend_from_slice(&read_buff[..size]);

                while i < line_buff.len() {
                    if line_buff[i] == b'\n' {
                        let line = &line_buff[begin..i];

                        if keyword.is_empty() && regex.is_empty() {
                            results.push(String::from_utf8_lossy(line).into());
                        }

                        if !keyword.is_empty() {
                            if let Some(result) = check_keyword(line, keyword) {
                                results.push(String::from_utf8_lossy(result).into());
                            }
                        }

                        if !regex.is_empty() {
                            if let Some(regex_results) = check_regex(line, regex)? {
                                for result in regex_results {
                                    results.push(String::from_utf8_lossy(result).into());
                                }
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
                break;
            }
        }
    }

    Ok(results)
}

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn file_normal(
    keyword: &[u8],
    regex: &[u8],
    input: Input,
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut results: Vec<String> = vec![];
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let mmap = unsafe { map_file(input)? };
    let mut begin = 0usize;
    let mut i = 0usize;

    while i <= mmap.len() {
        match memchr::memchr(b'\n', &mmap[i..]) {
            Some(0) => {
                break;
            }
            Some(pos) => {
                let end = i + pos;
                let line = &mmap[begin..end];

                if keyword.is_empty() && regex.is_empty() {
                    results.push(String::from_utf8_lossy(line).into());
                }

                if !keyword.is_empty() {
                    if let Some(result) = check_keyword(line, keyword) {
                        results.push(String::from_utf8_lossy(result).into());
                    }
                }

                if !regex.is_empty() {
                    if let Some(regex_results) = check_regex(line, regex)? {
                        for result in regex_results {
                            results.push(String::from_utf8_lossy(result).into());
                        }
                    }
                }

                begin = end + 1;
                i = begin;
            }
            None => {
                if begin < mmap.len() {
                    let line = &mmap[begin..];

                    if keyword.is_empty() && regex.is_empty() {
                        results.push(String::from_utf8_lossy(line).into());
                    }

                    if !keyword.is_empty() {
                        if let Some(result) = check_keyword(line, keyword) {
                            results.push(String::from_utf8_lossy(result).into());
                        }
                    }

                    if !regex.is_empty() {
                        if let Some(regex_results) = check_regex(line, regex)? {
                            for result in regex_results {
                                results.push(String::from_utf8_lossy(result).into());
                            }
                        }
                    }
                }
                break;
            }
        }
    }

    Ok(results)
}

/**
 * # Safety
 * Memory mapping a file is not a safe thing to do
*/
pub unsafe fn file_chunk_rayon(
    keyword: &[u8],
    regex: &[u8],
    input: Input,
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let file_size = input.metadata()?.len();
    if file_size == 0 {
        return Err(ReadInputError::EmptyFile.into());
    }
    let mmap = unsafe { map_file(input)? };
    let mmap = &mmap[..];

    mmap.split(|&b| b == b'\n')
        .filter_map(|line| {
            if keyword.is_empty() && regex.is_empty() {
                return Some(vec![line]);
            }

            if !keyword.is_empty() {
                if let Some(result) = check_keyword(line, keyword) {
                    return Some(vec![result]);
                }
            }

            if !regex.is_empty() {
                if let Ok(Some(results)) = check_regex(line, regex) {
                    return Some(results);
                }
            }

            None
        })
        .for_each(|lines| {
            let mut lock = results
                .lock()
                .expect("input/get_input.rs: Lock is poisoned");
            for line in lines {
                lock.push(String::from_utf8_lossy(line).into());
            }
            drop(lock);
        });

    let lock = results
        .lock()
        .expect("input/get_input.rs: Lock is poisoned");
    let vec = lock.to_vec();
    drop(lock);
    Ok(vec)
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
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let input = match input {
        Input::File(file) => Input::File(file),
        Input::Stdin(_) => {
            return Err(ReadInputError::CannotMapStdin.into());
        }
    };
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let file_size = input.metadata()?.len();
    if file_size == 0 {
        return Err(ReadInputError::EmptyFile.into());
    }
    let mmap = unsafe { map_file(input)? };
    let mmap = Arc::new(mmap);
    let keyword: Arc<&[u8]> = Arc::new(keyword);

    thread::scope({
        let results = Arc::clone(&results);
        move |scope| {
            for id in 0..num_workers {
                let results = Arc::clone(&results);
                let keyword = Arc::clone(&keyword);
                let mmap = mmap.clone();
                let mmap_size = mmap.len();
                let chunk_size = mmap_size / num_workers;
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
                                let line = &mmap[begin..end];
                                let mut lock = results
                                    .lock()
                                    .expect("input/get_input.rs: Lock is poisoned");

                                if keyword.is_empty() && regex.is_empty() {
                                    lock.push(String::from_utf8_lossy(line).into());
                                }

                                if !keyword.is_empty() {
                                    if let Some(result) = check_keyword(line, &keyword) {
                                        lock.push(String::from_utf8_lossy(result).into());
                                    }
                                }

                                if !regex.is_empty() {
                                    if let Ok(Some(regex_results)) = check_regex(line, regex) {
                                        for result in regex_results {
                                            lock.push(String::from_utf8_lossy(result).into());
                                        }
                                    }
                                }

                                drop(lock);

                                begin = end + 1;
                            }
                            None => {
                                let line = &mmap[begin..end];
                                let mut lock = results
                                    .lock()
                                    .expect("input/get_input.rs: Lock is poisoned");

                                if !line.is_empty() {
                                    if keyword.is_empty() && regex.is_empty() {
                                        lock.push(String::from_utf8_lossy(line).into());
                                    }

                                    if !keyword.is_empty() {
                                        if let Some(result) = check_keyword(line, &keyword) {
                                            lock.push(String::from_utf8_lossy(result).into());
                                        }
                                    }

                                    if !regex.is_empty() {
                                        if let Ok(Some(regex_results)) = check_regex(line, regex) {
                                            for result in regex_results {
                                                lock.push(String::from_utf8_lossy(result).into());
                                            }
                                        }
                                    }

                                    drop(lock);
                                }

                                break;
                            }
                        }
                    }
                });
            }
        }
    });

    let lock = results
        .lock()
        .expect("input/get_input.rs: Lock is poisoned");
    let vec = lock.to_vec();
    drop(lock);
    Ok(vec)
}

pub fn dedup_normal(lines: Vec<String>) -> Vec<String> {
    let hashset: HashSet<String> = lines.into_iter().collect();
    let vec: Vec<String> = hashset.into_iter().collect();
    vec
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
) -> Result<Vec<String>, Box<dyn error::Error>> {
    let keyword = keyword.as_bytes();
    let regex = regex.as_bytes();

    match input {
        // Use BufReader with stdin
        Input::Stdin(_) => stdin_normal(keyword, regex),
        // Use MemMap2 with with files
        Input::File(_) => match method {
            Method::Simple => unsafe { file_normal(keyword, regex, input) },
            Method::Rayon => unsafe { file_chunk_rayon(keyword, regex, input) },
            Method::StdThread => unsafe {
                file_chunk_std(keyword, regex, input, rayon::current_num_threads())
            },
        },
    }
}

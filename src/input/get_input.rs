
// pub mod split {
//     pub mod rayon {
//         use crate::{
//             common::{ReadInputError, check_keyword, check_regex, map_file},
//             input::Input,
//         };
//         use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
//         use std::{
//             collections::HashSet,
//             error,
//             sync::{Arc, Mutex},
//         };

//         /**
//          * # Safety
//          * Memory mapping a file is not a safe thing to do
//          */
//         pub unsafe fn file(
//             prefix: &[u8],
//             suffix: &[u8],
//             keyword: &[u8],
//             regex: &[u8],
//             input: Input,
//         ) -> Result<Vec<String>, Box<dyn error::Error>> {
//             let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
//             let input = match input {
//                 Input::File(file) => Input::File(file),
//                 Input::Stdin(_) => {
//                     return Err(ReadInputError::CannotMapStdin.into());
//                 }
//             };
//             let file_size = input.metadata()?.len();
//             if file_size == 0 {
//                 return Err(ReadInputError::EmptyFile.into());
//             }
//             let mmap = unsafe { map_file(input)? };
//             let mmap = &mmap[..];

//             mmap.split(|&b| b == b'\n')
//                 .par_bridge()
//                 .filter_map(|line| {
//                     if keyword.is_empty() && regex.is_empty() {
//                         return Some(vec![line]);
//                     }

//                     if !keyword.is_empty()
//                         && let Some(result) = check_keyword(line, keyword)
//                     {
//                         return Some(vec![result]);
//                     }

//                     if !regex.is_empty()
//                         && let Ok(Some(results)) = check_regex(line, regex)
//                     {
//                         return Some(results);
//                     }

//                     None
//                 })
//                 .for_each(|lines| {
//                     let mut lock = results
//                         .lock()
//                         .expect("input/get_input.rs: Lock is poisoned");
//                     for line in lines {
//                         let line = [prefix, line, suffix].concat();
//                         let line = line.as_slice();

//                         lock.push(String::from_utf8_lossy(line).into());
//                     }
//                     drop(lock);
//                 });

//             let lock = results
//                 .lock()
//                 .expect("input/get_input.rs: Lock is poisoned");
//             let vec = lock.to_vec();
//             drop(lock);
//             Ok(vec)
//         }

//         pub fn dedup_vec(lines: Vec<String>) -> Vec<String> {
//             let hashset: HashSet<String> = lines.par_iter().cloned().collect();
//             let vec: Vec<String> = hashset.par_iter().cloned().collect();
//             vec
//         }
//     }
// }

// pub mod chunk {
//     pub mod stdthread {
//         use crate::{
//             common::{ReadInputError, check_keyword, check_regex, map_file},
//             input::Input,
//         };
//         use std::{
//             error,
//             sync::{Arc, Mutex},
//             thread,
//         };

//         /**
//          * # Safety
//          * Memory mapping a file is not a safe thing to do
//          */
//         pub unsafe fn file(
//             prefix: &[u8],
//             suffix: &[u8],
//             keyword: &[u8],
//             regex: &[u8],
//             input: Input,
//             num_workers: usize,
//         ) -> Result<Vec<String>, Box<dyn error::Error>> {
//             let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
//             let input = match input {
//                 Input::File(file) => Input::File(file),
//                 Input::Stdin(_) => {
//                     return Err(ReadInputError::CannotMapStdin.into());
//                 }
//             };
//             let file_size = input.metadata()?.len();
//             if file_size == 0 {
//                 return Err(ReadInputError::EmptyFile.into());
//             }
//             let mmap = unsafe { map_file(input)? };
//             let mmap = Arc::new(mmap);
//             let keyword: Arc<&[u8]> = Arc::new(keyword);
//             let regex: Arc<&[u8]> = Arc::new(regex);

//             thread::scope({
//                 let results = Arc::clone(&results);
//                 move |scope| {
//                     for id in 0..num_workers {
//                         let results = Arc::clone(&results);
//                         let keyword = Arc::clone(&keyword);
//                         let regex = Arc::clone(&regex);
//                         let mmap = Arc::clone(&mmap);
//                         let mmap_size = mmap.len();
//                         let chunk_size = mmap_size / num_workers;
//                         let mut begin = id * chunk_size;
//                         let mut end = begin + chunk_size;

//                         scope.spawn(move || {
//                             let mmap = &mmap[..];

//                             if id > 0 {
//                                 if let Some(b) = memchr::memchr(b'\n', &mmap[begin..]) {
//                                     begin += b + 1;
//                                 } else {
//                                     return;
//                                 }
//                             }

//                             if end < mmap_size {
//                                 if let Some(b) = memchr::memchr(b'\n', &mmap[end..]) {
//                                     end += b + 1;
//                                 } else {
//                                     end = mmap_size;
//                                 }
//                             }

//                             while begin < end {
//                                 match memchr::memchr(b'\n', &mmap[begin..end]) {
//                                     Some(size) => {
//                                         let end = begin + size;
//                                         let line = [prefix, &mmap[begin..end], suffix].concat();
//                                         let line = line.as_slice();
//                                         let mut lock = results
//                                             .lock()
//                                             .expect("input/get_input.rs: Lock is poisoned");

//                                         if keyword.is_empty() && regex.is_empty() {
//                                             lock.push(String::from_utf8_lossy(line).into());
//                                         }

//                                         if !keyword.is_empty()
//                                             && let Some(result) = check_keyword(line, &keyword)
//                                         {
//                                             lock.push(String::from_utf8_lossy(result).into());
//                                         }

//                                         if !regex.is_empty()
//                                             && let Ok(Some(results)) = check_regex(line, &regex)
//                                         {
//                                             for result in results {
//                                                 lock.push(String::from_utf8_lossy(result).into());
//                                             }
//                                         }

//                                         drop(lock);
//                                         begin = end + 1;
//                                     }
//                                     None => {
//                                         let line = [prefix, &mmap[begin..end], suffix].concat();
//                                         let line = line.as_slice();
//                                         let mut lock = results
//                                             .lock()
//                                             .expect("input/get_input.rs: Lock is poisoned");

//                                         if !line.is_empty() {
//                                             if keyword.is_empty() && regex.is_empty() {
//                                                 lock.push(String::from_utf8_lossy(line).into());
//                                             }

//                                             if !keyword.is_empty()
//                                                 && let Some(result) = check_keyword(line, &keyword)
//                                             {
//                                                 lock.push(String::from_utf8_lossy(result).into());
//                                             }

//                                             if !regex.is_empty()
//                                                 && let Ok(Some(results)) = check_regex(line, &regex)
//                                             {
//                                                 for result in results {
//                                                     lock.push(
//                                                         String::from_utf8_lossy(result).into(),
//                                                     );
//                                                 }
//                                             }
//                                         }

//                                         drop(lock);
//                                     }
//                                 }
//                             }
//                         });
//                     }
//                 }
//             });

//             let lock = results
//                 .lock()
//                 .expect("input/get_input.rs: Lock is poisoned");
//             let vec = lock.to_vec();
//             drop(lock);
//             Ok(vec)
//         }
//     }

//     pub mod rayon {
//         use crate::{
//             common::{ReadInputError, check_keyword, check_regex, map_file},
//             input::Input,
//         };
//         use std::{
//             error,
//             sync::{Arc, Mutex},
//         };

//         /**
//          * # Safety
//          * Memory mapping a file is not a safe thing to do
//          */
//         pub unsafe fn file(
//             prefix: &[u8],
//             suffix: &[u8],
//             keyword: &[u8],
//             regex: &[u8],
//             input: Input,
//             num_workers: usize,
//         ) -> Result<Vec<String>, Box<dyn error::Error>> {
//             let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
//             let input = match input {
//                 Input::File(file) => Input::File(file),
//                 Input::Stdin(_) => {
//                     return Err(ReadInputError::CannotMapStdin.into());
//                 }
//             };
//             let file_size = input.metadata()?.len();
//             if file_size == 0 {
//                 return Err(ReadInputError::EmptyFile.into());
//             }
//             let mmap = unsafe { map_file(input)? };
//             let mmap = Arc::new(mmap);
//             let keyword: Arc<&[u8]> = Arc::new(keyword);
//             let regex: Arc<&[u8]> = Arc::new(regex);

//             rayon::scope({
//                 let results = Arc::clone(&results);
//                 move |scope| {
//                     for id in 0..num_workers {
//                         let results = Arc::clone(&results);
//                         let keyword = Arc::clone(&keyword);
//                         let regex = Arc::clone(&regex);
//                         let mmap = Arc::clone(&mmap);
//                         let mmap_size = mmap.len();
//                         let chunk_size = mmap_size / num_workers;
//                         let mut begin = id * chunk_size;
//                         let mut end = begin + chunk_size;

//                         scope.spawn(move |_| {
//                             let mmap = &mmap[..];

//                             if id > 0 {
//                                 if let Some(b) = memchr::memchr(b'\n', &mmap[begin..]) {
//                                     begin += b + 1;
//                                 } else {
//                                     return;
//                                 }
//                             }

//                             if end < mmap_size {
//                                 if let Some(b) = memchr::memchr(b'\n', &mmap[end..]) {
//                                     end += b + 1;
//                                 } else {
//                                     end = mmap_size;
//                                 }
//                             }

//                             while begin < end {
//                                 match memchr::memchr(b'\n', &mmap[begin..end]) {
//                                     Some(size) => {
//                                         let end = begin + size;
//                                         let line = [prefix, &mmap[begin..end], suffix].concat();
//                                         let line = line.as_slice();
//                                         let mut lock = results
//                                             .lock()
//                                             .expect("input/get_input.rs: Lock is poisoned");

//                                         if keyword.is_empty() && regex.is_empty() {
//                                             lock.push(String::from_utf8_lossy(line).into());
//                                         }

//                                         if !keyword.is_empty()
//                                             && let Some(result) = check_keyword(line, &keyword)
//                                         {
//                                             lock.push(String::from_utf8_lossy(result).into());
//                                         }

//                                         if !regex.is_empty()
//                                             && let Ok(Some(results)) = check_regex(line, &regex)
//                                         {
//                                             for result in results {
//                                                 lock.push(String::from_utf8_lossy(result).into());
//                                             }
//                                         }

//                                         begin = end + 1;
//                                     }
//                                     None => {
//                                         let line = [prefix, &mmap[begin..end], suffix].concat();
//                                         let line = line.as_slice();
//                                         let mut lock = results
//                                             .lock()
//                                             .expect("input/get_input.rs: Lock is poisoned");

//                                         if !line.is_empty() {
//                                             if keyword.is_empty() && regex.is_empty() {
//                                                 lock.push(String::from_utf8_lossy(line).into());
//                                             }

//                                             if !keyword.is_empty()
//                                                 && let Some(result) = check_keyword(line, &keyword)
//                                             {
//                                                 lock.push(String::from_utf8_lossy(result).into());
//                                             }
//                                         }

//                                         if !regex.is_empty()
//                                             && let Ok(Some(results)) = check_regex(line, &regex)
//                                         {
//                                             for result in results {
//                                                 lock.push(String::from_utf8_lossy(result).into());
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         });
//                     }
//                 }
//             });

//             let lock = results
//                 .lock()
//                 .expect("input/get_input.rs: Lock is poisoned");
//             let vec = lock.to_vec();
//             drop(lock);
//             Ok(vec)
//         }
//     }
// }

// /**
//  * # Safety
//  * Memory mapping a file is not a safe thing to do
// */
// #[allow(clippy::too_many_arguments)]
// pub unsafe fn input(
//     method: Method,
//     provider: Provider,
//     input: Input,
//     #[allow(unused)] stable: bool,
//     prefix: String,
//     suffix: String,
//     keyword: String,
//     regex: String,
// ) -> Result<Vec<String>, Box<dyn error::Error>> {
//     let prefix = prefix.as_bytes();
//     let suffix = suffix.as_bytes();
//     let keyword = keyword.as_bytes();
//     let regex = regex.as_bytes();

//     match input {
//         // Use BufReader with stdin
//         Input::Stdin(_) => simple::stdin(prefix, suffix, keyword, regex),
//         // Use MemMap2 with with files
//         Input::File(_) => match method {
//             Method::Simple =>
//             // # Safety
//             // Memory mapping a file is not a fail safe thing to do
//             unsafe { simple::file(prefix, suffix, keyword, regex, input) },
//             Method::Split => match provider {
//                 Provider::Rayon => unsafe {
//                     split::rayon::file(prefix, suffix, keyword, regex, input)
//                 },
//                 _ => unimplemented!(),
//             },
//             Method::Chunk => match provider {
//                 Provider::Rayon => unsafe {
//                     chunk::rayon::file(
//                         prefix,
//                         suffix,
//                         keyword,
//                         regex,
//                         input,
//                         rayon::current_num_threads(),
//                     )
//                 },
//                 Provider::StdThread => unsafe {
//                     chunk::stdthread::file(
//                         prefix,
//                         suffix,
//                         keyword,
//                         regex,
//                         input,
//                         rayon::current_num_threads(),
//                     )
//                 },
//             },
//         },
//     }
// }

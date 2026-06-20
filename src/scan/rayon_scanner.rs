use crate::{
    scan::{ConcurrencyMethod, Out, ScanProperties, Scanner},
    utils::stdout_util::create_stdout_buf_write,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::sync::{Arc, Mutex};

pub struct RayonScanner {
    pub method: ConcurrencyMethod,
}
impl Scanner for RayonScanner {
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        match self.method {
            ConcurrencyMethod::None => {
                unimplemented!()
            }
            ConcurrencyMethod::Split => {
                let get = scan_properties.get;
                let out = if get {
                    Out::Results(Arc::new(Mutex::new(Vec::new())))
                } else {
                    Out::Writer(Arc::new(Mutex::new(create_stdout_buf_write())))
                };

                let mmap = scan_properties.input.mmap().unwrap();
                let mmap = &mmap[..];

                mmap.split(|&b| b == b'\n')
                    .par_bridge()
                    .filter_map(|line| {
                        Some(
                            self.match_line(&scan_properties, line)
                                .into_iter()
                                .flatten(),
                        )
                    })
                    .for_each(|iter| {
                        let match_results = iter;
                        for result in match_results {
                            out.push_or_write(result);
                        }
                    });

                out.get_results()
            }
            ConcurrencyMethod::Chunk => {
                let input = &scan_properties.input;
                let prefix = scan_properties.prefix;
                let suffix = scan_properties.suffix;
                let get = scan_properties.get;
                let out = if get {
                    Out::Results(Arc::new(Mutex::new(Vec::new())))
                } else {
                    Out::Writer(Arc::new(Mutex::new(create_stdout_buf_write())))
                };
                let mmap = input.mmap().unwrap();
                let mmap = Arc::new(mmap);
                let mmap_size = mmap.len();
                let chunk_size = mmap_size / rayon::current_num_threads();
                rayon::scope({
                    let out = out.clone();
                    move |scope| {
                        for id in 0..rayon::current_num_threads() {
                            let out = out.clone();
                            let mmap = Arc::clone(&mmap);
                            let scan_properties = scan_properties.clone();

                            scope.spawn(move |_| {
                                let mut begin = id * chunk_size;
                                let mut end = begin + chunk_size;
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
                                        Some(size) => {
                                            let end = begin + size;
                                            let line = [prefix, &mmap[begin..end], suffix].concat();
                                            let line = line.as_slice();

                                            let match_results = self
                                                .match_line(&scan_properties, line)
                                                .into_iter()
                                                .flatten();

                                            for results in match_results {
                                                out.push_or_write(results);
                                            }

                                            begin = end + 1;
                                        }
                                        None => {
                                            let line = [prefix, &mmap[begin..end], suffix].concat();
                                            let line = line.as_slice();

                                            let match_results = self
                                                .match_line(&scan_properties, line)
                                                .into_iter()
                                                .flatten();

                                            for results in match_results {
                                                out.push_or_write(results);
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                });
                out.get_results()
            }
        }
    }
}

use std::{
    io::{BufWriter, Read, Write},
    sync::{Arc, Mutex},
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use strum_macros::EnumString;

use crate::{
    input::Input,
    matching::{MatchStrategyIterator, Matcher},
    utils::{
        self,
        stdout_util::{BufWriterImpl, create_stdout_buf_write},
    },
};

/// Concurrency providers
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ConcurrencyProvider {
    Rayon,
    StdThread,
}
#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ConcurrencyMethod {
    None,
    Split,
    Chunk,
}
pub enum Out<W>
where
    W: Write + Send,
{
    Results(Arc<Mutex<Vec<String>>>),
    Writer(Arc<Mutex<BufWriter<W>>>),
}
impl<W> Out<W>
where
    W: Write + Send,
{
    pub fn push_or_write(&self, bytes: &[u8]) {
        match self {
            Out::Results(results) => {
                let mut lock = results.lock().unwrap();
                lock.push(String::from_utf8_lossy(bytes).into());
            }
            Out::Writer(writer) => {
                let mut lock = writer.lock().unwrap();
                lock.write_all_with_newline(bytes).unwrap();
            }
        }
    }
    pub fn get_results(&self) -> Option<Vec<String>> {
        match self {
            Out::Results(results) => {
                let lock = results.lock().unwrap();
                let vec = lock.to_vec();
                Some(vec)
            }
            Out::Writer(_) => None,
        }
    }
}
pub struct ScanProperties<'a> {
    pub input: Box<dyn Input + Sync>,
    pub prefix: &'a [u8],
    pub matcher: Matcher,
    pub suffix: &'a [u8],
    pub get: bool,
}
pub trait Scanner {
    fn match_line<'a>(
        &self,
        scan_properties: &'a ScanProperties<'a>,
        line: &'a [u8],
    ) -> Option<MatchStrategyIterator<'a>> {
        scan_properties.matcher.best_match(line)
    }
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>>;
}
pub struct NoneScanner {}
impl Scanner for NoneScanner {
    fn match_line<'a>(
        &self,
        scan_properties: &'a ScanProperties<'a>,
        line: &'a [u8],
    ) -> Option<MatchStrategyIterator<'a>> {
        scan_properties.matcher.best_match(line)
    }
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        let input = &scan_properties.input;
        let prefix = scan_properties.prefix;
        let suffix = scan_properties.suffix;
        let get = scan_properties.get;
        let out = if get {
            Out::Results(Arc::new(Mutex::new(Vec::new())))
        } else {
            Out::Writer(Arc::new(Mutex::new(create_stdout_buf_write())))
        };
        let mut reader = input.create_read_buf().unwrap();
        let mut line_buf = utils::string_util::create_line_buf();
        let mut read_buf = utils::string_util::create_read_buf();
        let mut begin = 0usize;
        let mut i = 0usize;

        loop {
            match reader.read(&mut read_buf) {
                Ok(0) => {
                    if !line_buf.is_empty() {
                        let line = [prefix, &line_buf, suffix].concat();
                        let line = line.as_slice();

                        let match_results = self
                            .match_line(&scan_properties, line)
                            .into_iter()
                            .flatten();

                        for results in match_results {
                            out.push_or_write(results);
                        }
                    }

                    break;
                }
                Ok(pos) => {
                    line_buf.extend_from_slice(&read_buf[..pos]);

                    while i < line_buf.len() {
                        if line_buf[i] == b'\n' {
                            let line = [prefix, &line_buf[begin..i], suffix].concat();
                            let line = line.as_slice();

                            let match_results = self
                                .match_line(&scan_properties, line)
                                .into_iter()
                                .flatten();

                            for results in match_results {
                                out.push_or_write(results);
                            }
                            begin = i + 1;
                        }
                        i += 1;
                    }
                    if begin == 0 {
                        continue;
                    }

                    if begin < line_buf.len() {
                        line_buf.drain(0..begin);
                    } else {
                        line_buf.clear();
                    }
                }
                Err(_) => break,
            }
        }

        out.get_results()
    }
}

pub struct RayonScanner {
    method: ConcurrencyMethod,
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
                unimplemented!()
            }
        }
    }
}
pub struct StdThreadScanner {
    method: ConcurrencyMethod,
}
impl Scanner for StdThreadScanner {
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        None
    }
}
pub struct ScanMethod {
    concurrency_method: ConcurrencyMethod,
    concurrency_provider: ConcurrencyProvider,
}
impl ScanMethod {
    pub fn new(method: ConcurrencyMethod, provider: ConcurrencyProvider) -> Self {
        ScanMethod {
            concurrency_method: method,
            concurrency_provider: provider,
        }
    }
}
pub struct ScannerBuilder {}
impl ScannerBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(scan_method: ScanMethod) -> Box<dyn Scanner> {
        match scan_method.concurrency_method {
            ConcurrencyMethod::None => Box::new(NoneScanner {}),
            ConcurrencyMethod::Split => match scan_method.concurrency_provider {
                ConcurrencyProvider::Rayon => Box::new(RayonScanner {
                    method: scan_method.concurrency_method,
                }),
                ConcurrencyProvider::StdThread => {
                    unimplemented!()
                }
            },
            ConcurrencyMethod::Chunk => {
                unimplemented!()
            }
        }
    }
}

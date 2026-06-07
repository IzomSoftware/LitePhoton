use std::{
    default,
    io::{Read, Write},
};

use regex::bytes::Regex;
use strum_macros::EnumString;

use crate::{
    input::{Input, InputType},
    matching::{MatchStrategyIterator, Matcher},
    scan,
    utils::{self, stdout_util::BufWriterImpl},
};

/// Concurrency providers
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, Default, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ConcurrencyProvider {
    #[default]
    Rayon,
    StdThread,
}
#[derive(Debug, Default, PartialEq, EnumString, Clone)]
pub enum ConcurrencyMethod {
    None,
    #[default]
    Split,
    Chunk,
}
pub struct ScanProperties<'a> {
    pub input: Box<dyn Input>,
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
        let mut results: Vec<String> = vec![];
        let input = &scan_properties.input;
        let prefix = scan_properties.prefix;
        let suffix = scan_properties.suffix;
        let get = scan_properties.get;
        let mut reader = input.create_read_buf().unwrap();
        let mut writer = utils::stdout_util::create_stdout_buf_write();
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

                        if get {
                            results
                                .extend(match_results.map(|b| String::from_utf8_lossy(b).into()));
                        } else {
                            for result in match_results {
                                if writer.write_all_with_newline(result).is_err() {
                                    return None;
                                }
                            }
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

                            if get {
                                results.extend(
                                    match_results.map(|b| String::from_utf8_lossy(b).into()),
                                );
                            } else {
                                for result in match_results {
                                    if writer.write_all_with_newline(result).is_err() {
                                        return None;
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

                    if begin < line_buf.len() {
                        line_buf.drain(0..begin);
                    } else {
                        line_buf.clear();
                    }
                }
                Err(_) => break,
            }
        }
        if get && !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }
}

pub struct RayonScanner {}
impl Scanner for RayonScanner {
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        None
    }
}
pub struct StdThreadScanner {}
impl Scanner for StdThreadScanner {
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        None
    }
}
pub struct ScannerBuilder {}
impl ScannerBuilder {
    pub fn new(concurrency_method: ConcurrencyMethod) -> Box<dyn Scanner> {
        match concurrency_method {
            ConcurrencyMethod::None => Box::new(NoneScanner {}),
            ConcurrencyMethod::Split => unimplemented!(),
            ConcurrencyMethod::Chunk => unimplemented!(),
        }
    }
}

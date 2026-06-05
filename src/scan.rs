use regex::bytes::{Matches, Regex};
use strum_macros::EnumString;

pub enum ScanIterator<'a> {
    Keyword(std::iter::Once<&'a [u8]>),
    Regex(Matches<'a, 'a>),
}
impl<'a> Iterator for ScanIterator<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ScanIterator::Keyword(iterator) => iterator.next(),
            ScanIterator::Regex(iter) => iter.next().map(|b| b.as_bytes()),
        }
    }
}
pub trait Scanner {
    fn best_match<'a>(&'a self, line: &'a [u8]) -> Option<ScanIterator<'a>>;
}
pub enum ScanType {
    Keyword(Vec<u8>),
    Regex(Regex),
    Both(Vec<u8>, Regex),
}

struct KeywordScanner {
    keyword: Vec<u8>,
}
impl Scanner for KeywordScanner {
    fn best_match<'a>(&'a self, line: &'a [u8]) -> Option<ScanIterator<'a>> {
        let keyword = &self.keyword;
        if memchr::memmem::find(line, keyword).is_some() {
            return Some(ScanIterator::Keyword(std::iter::once(line)));
        }
        None
    }
}
struct RegexScanner {
    regex: Regex,
}
impl Scanner for RegexScanner {
    fn best_match<'a>(&'a self, line: &'a [u8]) -> Option<ScanIterator<'a>> {
        let regex = &self.regex;
        if regex.is_match(line) {
            return Some(ScanIterator::Regex(self.regex.find_iter(line)));
        }
        None
    }
}
struct BothScanner {
    pub keyword: Vec<u8>,
    pub regex: Regex,
}
impl Scanner for BothScanner {
    fn best_match<'a>(&'a self, line: &'a [u8]) -> Option<ScanIterator<'a>> {
        let keyword = &self.keyword;
        let regex = &self.regex;
        if memchr::memmem::find(line, keyword).is_some() {
            return Some(ScanIterator::Keyword(std::iter::once(line)));
        }
        if regex.is_match(line) {
            return Some(ScanIterator::Regex(self.regex.find_iter(line)));
        }
        None
    }
}
/// Concurrency providers
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Provider {
    Rayon,
    StdThread,
}

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Method {
    Simple,
    Split,
    Chunk,
}

pub struct ScannerBuilder;
impl ScannerBuilder {
    pub fn new(scan_type: ScanType) -> Box<dyn Scanner> {
        match scan_type {
            ScanType::Keyword(keyword) => Box::new(KeywordScanner { keyword }),
            ScanType::Regex(regex) => Box::new(RegexScanner { regex }),
            ScanType::Both(keyword, regex) => Box::new(BothScanner { keyword, regex }),
        }
    }
}

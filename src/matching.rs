use regex::bytes::{Matches, Regex};

pub enum MatchStrategyIterator<'a> {
    Keyword(std::iter::Once<&'a [u8]>),
    Regex(Matches<'a, 'a>),
}
impl<'a> Iterator for MatchStrategyIterator<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MatchStrategyIterator::Keyword(iterator) => iterator.next(),
            MatchStrategyIterator::Regex(iter) => iter.next().iter().next().map(|b| b.as_bytes()),
        }
    }
}
pub enum Matcher {
    Keyword(Vec<u8>),
    Regex(Regex),
    Both(Vec<u8>, Regex),
}
impl Matcher {
    pub fn best_match<'a>(&'a self, line: &'a [u8]) -> Option<MatchStrategyIterator<'a>> {
        match self {
            Matcher::Keyword(keyword) => {
                if memchr::memmem::find(line, keyword).is_some() {
                    return Some(MatchStrategyIterator::Keyword(std::iter::once(line)));
                }
            }
            Matcher::Regex(regex) => {
                if regex.is_match(line) {
                    return Some(MatchStrategyIterator::Regex(regex.find_iter(line)));
                }
            }
            Matcher::Both(keyword, regex) => {
                if memchr::memmem::find(line, keyword).is_some() {
                    return Some(MatchStrategyIterator::Keyword(std::iter::once(line)));
                }
                if regex.is_match(line) {
                    return Some(MatchStrategyIterator::Regex(regex.find_iter(line)));
                }
            }
        }
        None
    }
}

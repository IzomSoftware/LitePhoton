use std::collections::HashSet;

use regex::bytes::Regex;

pub fn create_line_buf() -> Vec<u8> {
    Vec::with_capacity(4 * 1024)
}
pub fn create_read_buf() -> [u8; 256 * 1024] {
    [0u8; 256 * 1024]
}
pub fn compile_regex(regex_str: &str) -> Result<Regex, regex::Error> {
    Regex::new(regex_str)
}
pub fn dedup_vec(lines: Vec<String>) -> Vec<String> {
    let hashset: HashSet<String> = lines.into_iter().collect();
    let vec: Vec<String> = hashset.into_iter().collect();
    vec
}

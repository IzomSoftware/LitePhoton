use std::{
    io::Read,
    sync::{Arc, Mutex},
};

use crate::{
    matching::MatchStrategyIterator,
    scan::{Out, ScanProperties, Scanner},
    utils::{self, stdout_util::create_stdout_buf_write},
};

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

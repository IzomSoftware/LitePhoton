pub mod none_scanner;
pub mod rayon_scanner;
pub mod stdthread_scanner;
use crate::{
    input::Input,
    matching::{MatchStrategyIterator, Matcher},
    scan::{none_scanner::NoneScanner, rayon_scanner::RayonScanner},
    utils::stdout_util::{BufWriterImpl},
};
use std::{
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;

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

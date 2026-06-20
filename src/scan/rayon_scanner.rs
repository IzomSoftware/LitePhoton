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
                unimplemented!()
            }
        }
    }
}

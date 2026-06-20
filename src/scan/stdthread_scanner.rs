use crate::scan::{ConcurrencyMethod, ScanProperties, Scanner};

pub struct StdThreadScanner {
    method: ConcurrencyMethod,
}
impl Scanner for StdThreadScanner {
    fn scan(&self, scan_properties: ScanProperties) -> Option<Vec<String>> {
        None
    }
}

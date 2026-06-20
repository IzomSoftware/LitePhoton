use crate::scan::{ConcurrencyMethod, ScanProperties, Scanner};

pub struct StdThreadScanner {
    _method: ConcurrencyMethod,
}
impl Scanner for StdThreadScanner {
    fn scan(&self, _scan_properties: ScanProperties) -> Option<Vec<String>> {
        None
    }
}

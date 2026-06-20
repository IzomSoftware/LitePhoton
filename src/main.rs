// use crate::{
//     argument_parser::Arguments,
//     common::{Method, Provider},
//     environment::Environment,
//     input::Input,
// };
// use log::info;
// use std::{path::PathBuf, str::FromStr};

use std::{path::PathBuf, str::FromStr};

use log::{info};

use crate::{
    argument_parser::ARGUMENTS,
    input::{InputBuilder, InputType},
    matching::Matcher,
    scan::{ConcurrencyMethod, ConcurrencyProvider, ScanMethod, ScanProperties, ScannerBuilder},
};

mod argument_parser;
mod logger;
pub mod input;
pub mod matching;
pub mod scan;
pub mod utils;


fn main() {
    let args = &*ARGUMENTS;

    logger::setup_logger(args.debug).expect("Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", args);

    let is_tty = !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !args.bypass_stdin_check;
    let (method, input) = if is_tty {
        (ConcurrencyMethod::None, vec![InputBuilder::new(InputType::Stdin)])
    } else {
        let mut inputs = vec![];
        for file in &args.file {
            inputs.push(InputBuilder::new(InputType::File(PathBuf::from(file))));
        }
        (ConcurrencyMethod::from_str(&args.method).expect("main.rs: Unexpected method"), inputs)
    };
    let provider =
        ConcurrencyProvider::from_str(&args.provider).expect("main.rs: Unexpected provider");

    for input in input {
        ScannerBuilder::new(ScanMethod::new(method.clone(), provider.clone())).scan(
            ScanProperties {
                get: false,
                input,
                matcher: Matcher::Keyword(args.keyword.as_bytes().to_vec()),
                prefix: args.prefix.as_bytes(),
                suffix: args.suffix.as_bytes(),
            },
        );
    }
}

// use crate::{
//     argument_parser::Arguments,
//     common::{Method, Provider},
//     environment::Environment,
//     input::Input,
// };
// use log::info;
// use std::{path::PathBuf, str::FromStr};

use std::str::FromStr;

use log::{error, info};

use crate::{
    argument_parser::ARGUMENTS, input::{InputBuilder, InputType}, matching::Matcher, scan::{ConcurrencyMethod, ConcurrencyProvider, ScanProperties, ScannerBuilder}
};

mod argument_parser;
pub mod input;
mod logger;
pub mod matching;
pub mod scan;
pub mod utils;

// /// Entry point
// #[allow(clippy::collapsible_else_if)]
// fn main() {
//     let args = Arguments::setup();

//     let env = Environment::setup(args);

//     logger::setup_logger(env.debug).expect("main.rs:: Cannot setup logger");

//     info!("Starting up LitePhoton with this environment: {:?}", env);

//     if is_tty {
//         if env.dedup {
//             let lines = unsafe {
//                 input::get_input::input(
//                     method.clone(),
//                     provider,
//                     Input::Stdin(()),
//                     env.stable,
//                     env.prefix.clone(),
//                     env.suffix.clone(),
//                     env.keyword.clone(),
//                     env.regex.clone(),
//                 )
//                 .expect("main.rs: Couldn't read stdin")
//             };

//             let results = if matches!(method, Method::Simple) {
//                 input::get_input::simple::dedup_vec(lines)
//             } else {
//                 input::get_input::split::rayon::dedup_vec(lines)
//             };

//             for line in results {
//                 println!("{line}")
//             }
//         } else {
//             unsafe {
//                 input::print_input::input(
//                     method,
//                     provider,
//                     Input::Stdin(()),
//                     env.stable,
//                     env.prefix.clone(),
//                     env.suffix.clone(),
//                     env.keyword.clone(),
//                     env.regex.clone(),
//                 )
//                 .expect("main.rs: Couldn't read stdin")
//             };
//         }
//     } else {
//         if env.dedup {
//             let mut lines = vec![];

//             for file in &env.file {
//                 lines.extend(
//                     unsafe {
//                         input::get_input::input(
//                             method.clone(),
//                             provider.clone(),
//                             Input::File(PathBuf::from(file)),
//                             env.stable,
//                             env.prefix.clone(),
//                             env.suffix.clone(),
//                             env.keyword.clone(),
//                             env.regex.clone(),
//                         )
//                         .expect("main.rs: Couldn't read stdin")
//                     }
//                     .iter()
//                     .cloned(),
//                 );
//             }

//             let results = if matches!(method, Method::Simple) {
//                 input::get_input::simple::dedup_vec(lines)
//             } else {
//                 input::get_input::split::rayon::dedup_vec(lines)
//             };
//             for line in results {
//                 println!("{line}")
//             }
//         } else {
//             for file in &env.file {
//                 unsafe {
//                     input::print_input::input(
//                         method.clone(),
//                         provider.clone(),
//                         Input::File(PathBuf::from(file)),
//                         env.stable,
//                         env.prefix.clone(),
//                         env.suffix.clone(),
//                         env.keyword.clone(),
//                         env.regex.clone(),
//                     )
//                     .expect("main.rs: Couldn't read file");
//                 }
//             }
//         }
//     }
// }
fn main() {
    let args = &*ARGUMENTS;

    logger::setup_logger(args.debug).expect("Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", args);

    let is_tty = !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !args.bypass_stdin_check;
    // let method = ConcurrencyMethod::from_str(&args.method).expect("main.rs: Unexpected method");
    // let provider =
        // ConcurrencyProvider::from_str(&args.provider).expect("main.rs: Unexpected provider");

    ScannerBuilder::new(ConcurrencyMethod::None).scan(ScanProperties {
        get: false,
        input: InputBuilder::new(InputType::Stdin),
        matcher: Matcher::Keyword("sasofaskfopia91".as_bytes().to_vec()),
        prefix: "hi".as_bytes(),
        suffix: "bye".as_bytes(),
    });
}

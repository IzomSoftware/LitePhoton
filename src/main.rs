use crate::argument_parser::{ARGUMENTS, Arguments};
use crate::common::Method;
use crate::environment::{ENVIRONMENT, Environment};
use crate::input::Input;
use log::info;
use std::path::PathBuf;
use std::str::FromStr;
mod argument_parser;
mod common;
mod environment;
mod input;
mod logger;
/// Entry point
fn main() {
    Arguments::setup().expect("main.rs: Couldn't parse args");

    Environment::setup(ARGUMENTS.get().expect("main.rs: Cannot get arguments"))
        .expect("main.rs: cannot set ENVIRONMENT. already initialized?");

    let env = ENVIRONMENT.get().expect("main.rs: Cannot get environment");

    logger::setup_logger(env.debug).expect("main.rs:: Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", env);

    if !env.dedup {
        if !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check {
            unsafe {
                input::print_input::input(
                    Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                    Input::Stdin(()),
                    env.stable,
                    env.keyword.clone(),
                )
                .expect("main.rs: Couldn't read stdin")
            };
        } else {
            for file in &env.file {
                unsafe {
                    input::print_input::input(
                        Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                        Input::File(PathBuf::from(file)),
                        env.stable,
                        env.keyword.clone(),
                    )
                    .expect("main.rs: Couldn't read file");
                }
            }
        }
    } else if !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check {
        let lines = unsafe {
            input::get_input::input(
                Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                Input::Stdin(()),
                env.stable,
                env.keyword.clone(),
            )
            .expect("main.rs: Couldn't read stdin")
        };
        let results = input::get_input::dedup_normal(lines);
        for line in results {
            println!("{line}")
        }
    } else {
        for file in &env.file {
            let lines = unsafe {
                input::get_input::input(
                    Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                    Input::File(PathBuf::from(file)),
                    env.stable,
                    env.keyword.clone(),
                )
                .expect("main.rs: Couldn't read file")
            };
            let results = input::get_input::dedup_normal(lines);
            for line in results {
                println!("{line}");
            }
        }
    }
}

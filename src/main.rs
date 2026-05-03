use crate::{
    argument_parser::Arguments,
    common::{Method, Provider},
    environment::Environment,
    input::Input,
};
use log::info;
use std::{path::PathBuf, str::FromStr};

mod argument_parser;
mod common;
mod environment;
mod input;
mod logger;

/// Entry point
fn main() {
    let args = Arguments::setup();

    let env = Environment::setup(args);

    logger::setup_logger(env.debug).expect("main.rs:: Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", env);

    if !env.dedup {
        if !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check {
            unsafe {
                input::print_input::input(
                    Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                    Provider::from_str(&env.provider)
                        .expect("main.rs: Provided provider not found"),
                    Input::Stdin(()),
                    env.stable,
                    env.keyword.clone(),
                    env.regex.clone(),
                )
                .expect("main.rs: Couldn't read stdin")
            };
        } else {
            for file in &env.file {
                unsafe {
                    input::print_input::input(
                        Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                        Provider::from_str(&env.provider)
                            .expect("main.rs: Provided provider not found"),
                        Input::File(PathBuf::from(file)),
                        env.stable,
                        env.keyword.clone(),
                        env.regex.clone(),
                    )
                    .expect("main.rs: Couldn't read file");
                }
            }
        }
    } else if !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check {
        let lines = unsafe {
            input::get_input::input(
                Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                Provider::from_str(&env.provider).expect("main.rs: Provided provider not found"),
                Input::Stdin(()),
                env.stable,
                env.keyword.clone(),
                env.regex.clone(),
            )
            .expect("main.rs: Couldn't read stdin")
        };
        let results = input::get_input::simple::dedup_vec(lines);
        for line in results {
            println!("{line}")
        }
    } else {
        for file in &env.file {
            let lines = unsafe {
                input::get_input::input(
                    Method::from_str(&env.method).expect("main.rs: Provided mode not found"),
                    Provider::from_str(&env.provider)
                        .expect("main.rs: Provided provider not found"),
                    Input::File(PathBuf::from(file)),
                    env.stable,
                    env.keyword.clone(),
                    env.regex.clone(),
                )
                .expect("main.rs: Couldn't read file")
            };
            let results = input::get_input::simple::dedup_vec(lines);
            for line in results {
                println!("{line}");
            }
        }
    }
}

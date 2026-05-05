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
#[allow(clippy::collapsible_else_if)]
fn main() {
    let args = Arguments::setup();

    let env = Environment::setup(args);

    logger::setup_logger(env.debug).expect("main.rs:: Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", env);

    let is_tty = !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check;
    let method = Method::from_str(&env.method).expect("main.rs: Unexpected method");
    let provider = Provider::from_str(&env.provider).expect("main.rs: Unexpected provider");

    if is_tty {
        if env.dedup {
            let lines = unsafe {
                input::get_input::input(
                    method.clone(),
                    provider,
                    Input::Stdin(()),
                    env.stable,
                    env.prefix.clone(),
                    env.suffix.clone(),
                    env.keyword.clone(),
                    env.regex.clone(),
                )
                .expect("main.rs: Couldn't read stdin")
            };

            let results = if matches!(method, Method::Simple) {
                input::get_input::simple::dedup_vec(lines)
            } else {
                input::get_input::split::rayon::dedup_vec(lines)
            };

            for line in results {
                println!("{line}")
            }
        } else {
            unsafe {
                input::print_input::input(
                    method,
                    provider,
                    Input::Stdin(()),
                    env.stable,
                    env.prefix.clone(),
                    env.suffix.clone(),
                    env.keyword.clone(),
                    env.regex.clone(),
                )
                .expect("main.rs: Couldn't read stdin")
            };
        }
    } else {
        if env.dedup {
            let mut lines = vec![];

            for file in &env.file {
                lines.extend(
                    unsafe {
                        input::get_input::input(
                            method.clone(),
                            provider.clone(),
                            Input::File(PathBuf::from(file)),
                            env.stable,
                            env.prefix.clone(),
                            env.suffix.clone(),
                            env.keyword.clone(),
                            env.regex.clone(),
                        )
                        .expect("main.rs: Couldn't read stdin")
                    }
                    .iter()
                    .cloned(),
                );
            }

            let results = if matches!(method, Method::Simple) {
                input::get_input::simple::dedup_vec(lines)
            } else {
                input::get_input::split::rayon::dedup_vec(lines)
            };
            for line in results {
                println!("{line}")
            }
        } else {
            for file in &env.file {
                unsafe {
                    input::print_input::input(
                        method.clone(),
                        provider.clone(),
                        Input::File(PathBuf::from(file)),
                        env.stable,
                        env.prefix.clone(),
                        env.suffix.clone(),
                        env.keyword.clone(),
                        env.regex.clone(),
                    )
                    .expect("main.rs: Couldn't read file");
                }
            }
        }
    }
}

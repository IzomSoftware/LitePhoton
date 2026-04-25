use crate::argument_parser::{ARGUMENTS, Arguments};
use crate::environment::{ENVIRONMENT, Environment};
use crate::input::Input;
use crate::read_util::Mode;
use log::info;
use std::path::PathBuf;
use std::str::FromStr;
mod argument_parser;
mod environment;
mod input;
mod logger;
mod read_util;

/// Entry point
fn main() {
    Arguments::setup().expect("Couldn't parse args");

    Environment::setup(ARGUMENTS.get().expect("main.rs: Cannot get arguments"))
            .expect("environment.rs: cannot set ENVIRONMENT. already initialized?");

    let env = ENVIRONMENT.get().expect("main.rs: Cannot get environment");

    logger::setup_logger(env.debug).expect("main.rs:: Cannot setup logger");

    info!("Starting up LitePhoton with this environment: {:?}", env);

    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) && !env.bypass_stdin_check {
        unsafe {
            read_util::read_input(
                Mode::from_str(&env.method).expect("main.rs: Provided mode not found"),
                Input::Stdin(()),
                env.stable,
                env.keyword.clone(),
            ).expect("Couldn't read stdin")
        };
    } else {
        for file in &env.file {
            unsafe {
                read_util::read_input(
                    Mode::from_str(&env.method).expect("main.rs: Provided mode not found"),
                    Input::File(PathBuf::from(file)),
                    env.stable,
                    env.keyword.clone(),
                ).expect("Couldn't read file");
            }
        }
    }
}

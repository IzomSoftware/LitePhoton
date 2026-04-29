use crate::argument_parser::Arguments;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Environment {
    pub debug: bool,
    pub bypass_stdin_check: bool,
    pub stable: bool,
    pub file: Vec<String>,
    pub keyword: String,
    pub dedup: bool,
    pub method: String,
}

impl Environment {
    fn get(args: &Arguments) -> Environment {
        Environment {
            debug: args.debug,
            bypass_stdin_check: args.bypass_stdin_check,
            stable: args.stable,
            file: args.file.clone(),
            keyword: args.keyword.clone(),
            dedup: args.dedup,
            method: args.method.clone(),
        }
    }
    pub fn setup(args: &Arguments) -> Result<(), Environment> {
        ENVIRONMENT
            .set(Self::get(args))
    }
}

pub static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

use crate::argument_parser::Arguments;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Environment {
    pub debug: bool,
    pub bypass_stdin_check: bool,
    pub stable: bool,
    pub dedup: bool,
    pub file: Vec<String>,
    pub keyword: String,
    pub regex: String,
    pub method: String,
    pub provider: String,
}

impl Environment {
    fn get(args: &Arguments) -> Environment {
        Environment {
            debug: args.debug,
            bypass_stdin_check: args.bypass_stdin_check,
            stable: args.stable,
            dedup: args.dedup,
            file: args.file.clone(),
            keyword: args.keyword.clone(),
            regex: args.regex.clone(),
            method: args.method.clone(),
            provider: args.provider.clone(),
        }
    }
    pub fn setup(args: &Arguments) -> &Environment {
        ENVIRONMENT.get_or_init(|| Self::get(args))
    }
}

pub static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

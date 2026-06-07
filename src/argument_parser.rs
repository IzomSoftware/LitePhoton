use clap::Parser;
use std::sync::LazyLock;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short, long, default_value = "false")]
    pub config: bool,
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
    #[arg(short, long, default_value = "false")]
    pub bypass_stdin_check: bool,
    #[arg(short, long, action = clap::ArgAction::Set, default_value = "true")]
    pub stable: bool,
    #[arg(short, long, default_value = "false")]
    pub dedup: bool,
    #[arg(short, long, num_args = 1..,default_value = "")]
    pub file: Vec<String>,
    #[arg(short, long, default_value = "")]
    pub keyword: String,
    #[arg(short, long, default_value = "")]
    pub regex: String,
    #[arg(short, long, default_value = "")]
    pub prefix: String,
    #[arg(short, long, default_value = "")]
    pub suffix: String,
    #[arg(short, long, default_value = "split")]
    pub method: String,
    #[arg(short, long, default_value = "rayon")]
    pub provider: String,
    // unnecessary because tty is different from stdin
    // #[arg(value_parser)]
    // pub last: Vec<String>,
}

impl Arguments {
    fn lowercase(mut self) -> Self {
        self.method = self.method.to_lowercase();
        self.provider = self.provider.to_lowercase();
        self
    }
}

pub static ARGUMENTS: LazyLock<Arguments> = LazyLock::new(|| Arguments::parse().lowercase());

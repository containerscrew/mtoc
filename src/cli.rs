use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    about = "mtoc",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Git markdown table of contents generator. ",
    arg_required_else_help = false
)]

pub struct Args {
    #[arg(
        short = 'd',
        long = "directory",
        help = "Directory to search for markdown files",
        default_value = ".",
        required = false
    )]
    pub directory: String,
}

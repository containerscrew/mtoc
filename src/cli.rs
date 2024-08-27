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
        short = 'f',
        long = "file-name",
        help = "Name of the markdown file to generate the table of contents for",
        default_value = "README.md",
        required = false
    )]
    pub file_name: String,
}

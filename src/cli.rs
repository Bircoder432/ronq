use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ronq")]
pub struct Args {
    /// Path to the RON file; if not provided, reads from stdin
    #[arg(long, short)]
    pub file: Option<String>,

    /// Dot-separated key path
    pub key: Option<String>,
}

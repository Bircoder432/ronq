mod cli;
mod query;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    let value = query::load_value(&args.file);

    let result = if let Some(path) = args.key.as_deref() {
        query::get_path(&value, path)
    } else {
        Some(&value)
    };

    if let Some(val) = result {
        query::pretty_print(val, 0);
    } else {
        eprintln!("Path not found");
    }
}

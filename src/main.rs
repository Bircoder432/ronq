mod cli;
mod query;
mod writer;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    let mut value = query::load_value(&args.file);

    if let (Some(path), Some(write_val)) = (args.key.as_deref(), args.write.as_deref()) {
        let typed_val = writer::parse_typed_value(write_val);
        if writer::set_path(&mut value, path, typed_val) {
            println!("Value updated!");
            if let Some(file) = &args.file {
                std::fs::write(file, ron::to_string(&value).unwrap())
                    .expect("Failed to write file");
            }
        } else {
            eprintln!("Failed to update path");
        }
    } else {
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
}

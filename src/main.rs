use clap::Parser;
use colored::*;
use ron::Value;
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "rq")]
struct Args {
    key: String,

    #[arg(short, long)]
    file: Option<String>,
}

fn get_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for key in path.split('.') {
        current = match current {
            Value::Map(map) => map.get(&Value::String(key.to_owned()))?,
            _ => return None,
        };
    }
    Some(current)
}

fn pretty_print(value: &Value, indent: usize) {
    let padding = "  ".repeat(indent);

    match value {
        Value::String(s) => println!("{}{}", padding, s.green()),
        Value::Bool(b) => println!("{}{}", padding, b.to_string().cyan()),

        Value::Number(n) => match n {
            ron::Number::I8(i) => println!("{}{}", padding, i.to_string().yellow()),
            ron::Number::I16(i) => println!("{}{}", padding, i.to_string().yellow()),
            ron::Number::I32(i) => println!("{}{}", padding, i.to_string().yellow()),
            ron::Number::I64(i) => println!("{}{}", padding, i.to_string().yellow()),
            ron::Number::U8(u) => println!("{}{}", padding, u.to_string().yellow()),
            ron::Number::U16(u) => println!("{}{}", padding, u.to_string().yellow()),
            ron::Number::U32(u) => println!("{}{}", padding, u.to_string().yellow()),
            ron::Number::U64(u) => println!("{}{}", padding, u.to_string().yellow()),

            ron::Number::F32(f) => println!("{}{}", padding, f.0.to_string().yellow()),
            ron::Number::F64(f) => println!("{}{}", padding, f.0.to_string().yellow()),
            _ => println!("{}{}", padding, "<unknown number>".red()),
        },

        Value::Seq(seq) => {
            println!("{}[", padding.blue());
            for v in seq {
                pretty_print(v, indent + 1);
            }
            println!("{}]", padding.blue());
        }

        Value::Map(map) => {
            println!("{}{{", padding.blue());
            for (k, v) in map.iter() {
                if let Value::String(key) = k {
                    print!("{}  {} = ", padding, key.bold());
                    match v {
                        Value::Map(_) | Value::Seq(_) => {
                            println!();
                            pretty_print(v, indent + 2);
                        }
                        _ => pretty_print(v, 0),
                    }
                }
            }
            println!("{}}}", padding.blue());
        }

        _ => println!("{}{:?}", padding, value),
    }
}

fn main() {
    let args = Args::parse();

    let content = if let Some(file) = args.file {
        fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("{}", "Failed to read file".red());
            std::process::exit(1);
        })
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_else(|_| {
            eprintln!("{}", "Failed to read stdin".red());
            std::process::exit(1);
        });
        buf
    };

    let value: Value = ron::from_str(&content).unwrap_or_else(|e| {
        eprintln!("{} {}", "Failed to parse RON:".red(), e.to_string().red());
        std::process::exit(1);
    });

    if let Some(val) = get_value(&value, &args.key) {
        pretty_print(val, 0);
    } else {
        eprintln!("{} '{}'", "Key not found".red(), args.key.red());
    }
}

use colored::Colorize;
use regex::Regex;
use ron::Value;
use std::fs;
use std::io::{self, Read};

pub fn load_value(file: &Option<String>) -> Value {
    let content = if let Some(f) = file {
        fs::read_to_string(f).expect("Failed to read file")
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read stdin");
        buf
    };

    ron::from_str(&content).expect("Failed to parse RON")
}

pub fn get_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    let re = Regex::new(r"([^\[\]]+)|\[(\d+)\]").unwrap();

    for part in path.split('.') {
        let mut temp = current;

        for cap in re.captures_iter(part) {
            if let Some(key_match) = cap.get(1) {
                let key = key_match.as_str();
                if let Value::Map(map) = temp {
                    temp = map.get(&Value::String(key.to_string()))?;
                } else {
                    return None;
                }
            } else if let Some(idx_match) = cap.get(2) {
                let idx: usize = idx_match.as_str().parse().ok()?;
                if let Value::Seq(seq) = temp {
                    temp = seq.get(idx)?;
                } else {
                    return None;
                }
            }
        }

        current = temp;
    }

    Some(current)
}

pub fn pretty_print(value: &Value, indent: usize) {
    let pad = " ".repeat(indent);

    match value {
        Value::Map(map) => {
            println!("{}{{", pad);
            for (k, v) in map.iter() {
                if let Value::String(key) = k {
                    print!("{}  {}: ", pad, key.cyan());
                    pretty_print(v, indent + 2);
                } else {
                    print!("{}  <non-string key>: ", pad);
                    pretty_print(v, indent + 2);
                }
            }
            println!("{}}}", pad);
        }
        Value::Seq(seq) => {
            println!("{}[", pad);
            for v in seq {
                pretty_print(v, indent + 2);
            }
            println!("{}]", pad);
        }
        Value::Number(n) => println!("{}{}", pad, n.into_f64().to_string().yellow()),
        Value::Bool(b) => println!("{}{}", pad, b.to_string().green()),
        Value::String(s) => println!("{}{}", pad, s),
        _ => println!("{}<unknown>", pad),
    }
}

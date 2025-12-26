use regex::Regex;
use ron::Value;

pub fn parse_typed_value(s: &str) -> Value {
    if let Ok(i) = s.parse::<i64>() {
        Value::Number(i.into())
    } else if let Ok(f) = s.parse::<f64>() {
        Value::Number(f.into())
    } else if let Ok(b) = s.parse::<bool>() {
        Value::Bool(b)
    } else {
        Value::String(s.to_string())
    }
}

pub fn set_path(value: &mut Value, path: &str, new_val: Value) -> bool {
    let re = Regex::new(r"([^\[\]]+)|\[(\d+)\]").unwrap();
    let mut current = value;

    let parts: Vec<&str> = path.split('.').collect();

    for (i, part) in parts.iter().enumerate() {
        let mut temp = current;
        for cap in re.captures_iter(part) {
            if let Some(key_match) = cap.get(1) {
                let key = Value::String(key_match.as_str().to_string());
                if let Value::Map(map) = temp {
                    if i == parts.len() - 1 {
                        map.insert(key, new_val);
                        return true;
                    }
                    temp = map.get_mut(&key).unwrap();
                } else {
                    return false;
                }
            } else if let Some(idx_match) = cap.get(2) {
                let idx: usize = idx_match.as_str().parse().ok().unwrap();
                if let Value::Seq(seq) = temp {
                    if i == parts.len() - 1 {
                        if idx < seq.len() {
                            seq[idx] = new_val;
                            return true;
                        } else {
                            return false;
                        }
                    }
                    temp = seq.get_mut(idx).unwrap();
                } else {
                    return false;
                }
            }
        }
        current = temp;
    }
    false
}

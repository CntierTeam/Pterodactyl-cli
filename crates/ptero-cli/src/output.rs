use serde::Serialize;
use serde_json::Value;

pub fn print_result<T: Serialize>(json: bool, value: &T) {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
        );
    } else {
        match serde_json::to_value(value) {
            Ok(Value::String(s)) => println!("{s}"),
            Ok(Value::Null) => println!("ok"),
            Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default()),
            Err(_) => println!("ok"),
        }
    }
}

pub fn print_ok(json: bool, msg: &str) {
    if json {
        println!("{}", serde_json::json!({ "ok": true, "message": msg }));
    } else {
        println!("{msg}");
    }
}

pub fn parse_json_body(s: &str) -> anyhow::Result<Value> {
    Ok(serde_json::from_str(s)?)
}

pub fn parse_permissions(s: &str) -> Vec<String> {
    s.split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

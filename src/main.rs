mod json;

use json::{JsonValue, parse, stringify};
use std::collections::HashMap;

fn main() {
    println!("=== JSON Parser and Stringifier Demo ===\n");

    // Parse examples
    let examples = vec![
        "null",
        "true",
        "false",
        "42",
        "3.14",
        r#""Hello, World!""#,
        "[1, 2, 3, 4, 5]",
        r#"{"name": "Alice", "age": 30}"#,
        r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#,
    ];

    println!("Parsing examples:");
    for example in &examples {
        match parse(example) {
            Ok(value) => println!("✓ {} -> {:?}", example, value),
            Err(e) => println!("✗ {} -> Error: {:?}", example, e),
        }
    }

    println!("\nStringifying examples:");
    let values = vec![
        JsonValue::Null,
        JsonValue::Bool(true),
        JsonValue::Number(42.0),
        JsonValue::String("Hello, World!".to_string()),
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]),
    ];

    for value in &values {
        let json = stringify(value);
        println!("✓ {:?} -> {}", value, json);
    }

    // Complex object example
    println!("\nComplex object example:");
    let mut person = HashMap::new();
    person.insert("name".to_string(), JsonValue::String("Alice".to_string()));
    person.insert("age".to_string(), JsonValue::Number(30.0));
    person.insert("active".to_string(), JsonValue::Bool(true));

    let mut scores = HashMap::new();
    scores.insert("math".to_string(), JsonValue::Number(95.0));
    scores.insert("science".to_string(), JsonValue::Number(87.0));
    person.insert("scores".to_string(), JsonValue::Object(scores));

    let complex = JsonValue::Object(person);
    let json_str = stringify(&complex);
    println!("Complex object: {}", json_str);

    // Round-trip test
    println!("\nRound-trip test:");
    if let Ok(parsed_again) = parse(&json_str) {
        println!("✓ Round-trip successful: {}", stringify(&parsed_again));
    } else {
        println!("✗ Round-trip failed");
    }
}

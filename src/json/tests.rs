#[cfg(test)]
mod tests {
    use crate::json::{JsonValue, ParseError, parse, stringify};
    use std::collections::HashMap;

    #[test]
    fn test_parse_null() {
        let result = parse("null");
        assert_eq!(result, Ok(JsonValue::Null));
    }

    #[test]
    fn test_parse_true() {
        let result = parse("true");
        assert_eq!(result, Ok(JsonValue::Bool(true)));
    }

    #[test]
    fn test_parse_false() {
        let result = parse("false");
        assert_eq!(result, Ok(JsonValue::Bool(false)));
    }

    #[test]
    fn test_parse_number_integer() {
        let result = parse("42");
        assert_eq!(result, Ok(JsonValue::Number(42.0)));
    }

    #[test]
    fn test_parse_number_float() {
        let result = parse("3.14");
        assert_eq!(result, Ok(JsonValue::Number(3.14)));
    }

    #[test]
    fn test_parse_number_negative() {
        let result = parse("-123");
        assert_eq!(result, Ok(JsonValue::Number(-123.0)));
    }

    #[test]
    fn test_parse_string_simple() {
        let result = parse(r#""hello""#);
        assert_eq!(result, Ok(JsonValue::String("hello".to_string())));
    }

    #[test]
    fn test_parse_string_with_escape() {
        let result = parse(r#""hello \"world\"""#);
        assert_eq!(result, Ok(JsonValue::String("hello \"world\"".to_string())));
    }

    #[test]
    fn test_parse_array_empty() {
        let result = parse("[]");
        assert_eq!(result, Ok(JsonValue::Array(vec![])));
    }

    #[test]
    fn test_parse_array_with_elements() {
        let result = parse("[1, 2, 3]");
        let expected = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_array_nested() {
        let result = parse("[1, [2, 3], 4]");
        let expected = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Array(vec![JsonValue::Number(2.0), JsonValue::Number(3.0)]),
            JsonValue::Number(4.0),
        ]);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_object_empty() {
        let result = parse("{}");
        assert_eq!(result, Ok(JsonValue::Object(HashMap::new())));
    }

    #[test]
    fn test_parse_object_simple() {
        let result = parse(r#"{"key": "value"}"#);
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(result, Ok(JsonValue::Object(expected)));
    }

    #[test]
    fn test_parse_object_multiple() {
        let result = parse(r#"{"a": 1, "b": 2}"#);
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), JsonValue::Number(1.0));
        expected.insert("b".to_string(), JsonValue::Number(2.0));
        assert_eq!(result, Ok(JsonValue::Object(expected)));
    }

    #[test]
    fn test_parse_complex() {
        let json = r#"{
            "name": "John Doe",
            "age": 30,
            "active": true,
            "scores": [95, 87, 91],
            "address": {
                "street": "123 Main St",
                "city": "Anytown"
            },
            "notes": null
        }"#;

        let result = parse(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stringify_null() {
        let value = JsonValue::Null;
        assert_eq!(stringify(&value), "null");
    }

    #[test]
    fn test_stringify_bool() {
        assert_eq!(stringify(&JsonValue::Bool(true)), "true");
        assert_eq!(stringify(&JsonValue::Bool(false)), "false");
    }

    #[test]
    fn test_stringify_number() {
        assert_eq!(stringify(&JsonValue::Number(42.0)), "42");
        assert_eq!(stringify(&JsonValue::Number(3.14)), "3.14");
    }

    #[test]
    fn test_stringify_string() {
        assert_eq!(
            stringify(&JsonValue::String("hello".to_string())),
            r#""hello""#
        );
    }

    #[test]
    fn test_stringify_string_with_quotes() {
        assert_eq!(
            stringify(&JsonValue::String("hello \"world\"".to_string())),
            r#""hello \"world\"""#
        );
    }

    #[test]
    fn test_stringify_array() {
        let value = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(stringify(&value), "[1,2,3]");
    }

    #[test]
    fn test_stringify_object() {
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), JsonValue::String("value".to_string()));
        let value = JsonValue::Object(obj);
        assert_eq!(stringify(&value), r#"{"key":"value"}"#);
    }

    #[test]
    fn test_round_trip() {
        let original = r#"{"name":"Alice","age":25,"active":true,"scores":[90,85,92]}"#;
        let parsed = parse(original).unwrap();
        let stringified = stringify(&parsed);
        let reparsed = parse(&stringified).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_parse_error_unexpected_char() {
        let result = parse("{invalid}");
        assert!(matches!(result, Err(ParseError::UnexpectedChar(_, _))));
    }

    #[test]
    fn test_parse_error_unexpected_end() {
        let result = parse("{");
        assert!(matches!(result, Err(ParseError::UnexpectedEndOfInput)));
    }
}


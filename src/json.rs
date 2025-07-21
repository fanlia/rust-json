use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedChar(char, usize),
    UnexpectedEndOfInput,
    InvalidNumber(String),
    InvalidEscape(String),
    InvalidUnicodeEscape(String),
}

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.to_string(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.pos < self.input.len() {
            return Err(ParseError::UnexpectedChar(
                self.current_char().unwrap(),
                self.pos,
            ));
        }
        Ok(value)
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        self.skip_whitespace();

        match self.current_char() {
            Some('n') => self.parse_null(),
            Some('t') => self.parse_true(),
            Some('f') => self.parse_false(),
            Some('"') => self.parse_string(),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(c) if c.is_ascii_digit() || c == '-' => self.parse_number(),
            Some(c) => Err(ParseError::UnexpectedChar(c, self.pos)),
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, ParseError> {
        if self.input[self.pos..].starts_with("null") {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            Err(ParseError::UnexpectedChar(
                self.current_char().unwrap(),
                self.pos,
            ))
        }
    }

    fn parse_true(&mut self) -> Result<JsonValue, ParseError> {
        if self.input[self.pos..].starts_with("true") {
            self.pos += 4;
            Ok(JsonValue::Bool(true))
        } else {
            Err(ParseError::UnexpectedChar(
                self.current_char().unwrap(),
                self.pos,
            ))
        }
    }

    fn parse_false(&mut self) -> Result<JsonValue, ParseError> {
        if self.input[self.pos..].starts_with("false") {
            self.pos += 5;
            Ok(JsonValue::Bool(false))
        } else {
            Err(ParseError::UnexpectedChar(
                self.current_char().unwrap(),
                self.pos,
            ))
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, ParseError> {
        self.advance(); // Skip opening quote
        let mut result = String::new();

        while let Some(c) = self.current_char() {
            match c {
                '"' => {
                    self.advance();
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    self.advance();
                    match self.current_char() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\x08'),
                        Some('f') => result.push('\x0c'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            self.advance();
                            let mut hex = String::new();
                            for _ in 0..4 {
                                if let Some(h) = self.current_char() {
                                    hex.push(h);
                                    self.advance();
                                } else {
                                    return Err(ParseError::InvalidUnicodeEscape(hex));
                                }
                            }
                            if let Ok(code_point) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(code_point) {
                                    result.push(ch);
                                } else {
                                    return Err(ParseError::InvalidUnicodeEscape(hex));
                                }
                            } else {
                                return Err(ParseError::InvalidUnicodeEscape(hex));
                            }
                            continue;
                        }
                        Some(_) => return Err(ParseError::InvalidEscape(format!("\\{}", c))),
                        None => return Err(ParseError::UnexpectedEndOfInput),
                    }
                    self.advance();
                }
                _ => {
                    result.push(c);
                    self.advance();
                }
            }
        }

        Err(ParseError::UnexpectedEndOfInput)
    }

    fn parse_number(&mut self) -> Result<JsonValue, ParseError> {
        let start = self.pos;

        if self.current_char() == Some('-') {
            self.advance();
        }

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if self.current_char() == Some('.') {
            self.advance();
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if self.current_char() == Some('e') || self.current_char() == Some('E') {
            self.advance();
            if self.current_char() == Some('+') || self.current_char() == Some('-') {
                self.advance();
            }
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str = &self.input[start..self.pos];
        match num_str.parse::<f64>() {
            Ok(num) => Ok(JsonValue::Number(num)),
            Err(_) => Err(ParseError::InvalidNumber(num_str.to_string())),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        self.advance(); // Skip '['
        self.skip_whitespace();

        let mut elements = Vec::new();

        if self.current_char() == Some(']') {
            self.advance();
            return Ok(JsonValue::Array(elements));
        }

        loop {
            let value = self.parse_value()?;
            elements.push(value);
            self.skip_whitespace();

            match self.current_char() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                }
                Some(']') => {
                    self.advance();
                    return Ok(JsonValue::Array(elements));
                }
                Some(c) => return Err(ParseError::UnexpectedChar(c, self.pos)),
                None => return Err(ParseError::UnexpectedEndOfInput),
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        self.advance(); // Skip '{'
        self.skip_whitespace();

        let mut map = HashMap::new();

        if self.current_char() == Some('}') {
            self.advance();
            return Ok(JsonValue::Object(map));
        }

        loop {
            let key = match self.parse_value()? {
                JsonValue::String(s) => s,
                _ => {
                    return Err(ParseError::UnexpectedChar(
                        self.current_char().unwrap(),
                        self.pos,
                    ));
                }
            };

            self.skip_whitespace();

            if self.current_char() != Some(':') {
                return Err(ParseError::UnexpectedChar(
                    self.current_char().unwrap(),
                    self.pos,
                ));
            }
            self.advance();

            self.skip_whitespace();
            let value = self.parse_value()?;
            map.insert(key, value);

            self.skip_whitespace();

            match self.current_char() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                }
                Some('}') => {
                    self.advance();
                    return Ok(JsonValue::Object(map));
                }
                Some(c) => return Err(ParseError::UnexpectedChar(c, self.pos)),
                None => return Err(ParseError::UnexpectedEndOfInput),
            }
        }
    }
}

pub fn parse(input: &str) -> Result<JsonValue, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse()
}

pub fn stringify(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(true) => "true".to_string(),
        JsonValue::Bool(false) => "false".to_string(),
        JsonValue::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        JsonValue::String(s) => {
            let mut result = String::new();
            result.push('"');
            for c in s.chars() {
                match c {
                    '"' => result.push_str("\\\""),
                    '\\' => result.push_str("\\\\"),
                    '\x08' => result.push_str("\\b"),
                    '\x0c' => result.push_str("\\f"),
                    '\n' => result.push_str("\\n"),
                    '\r' => result.push_str("\\r"),
                    '\t' => result.push_str("\\t"),
                    _ if c.is_control() => {
                        let code = c as u32;
                        result.push_str(&format!("\\u{:04x}", code));
                    }
                    _ => result.push(c),
                }
            }
            result.push('"');
            result
        }
        JsonValue::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(stringify).collect();
            format!("[{}]", elements.join(","))
        }
        JsonValue::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}:{}",
                        stringify(&JsonValue::String(k.clone())),
                        stringify(v)
                    )
                })
                .collect();
            format!("{{{}}}", pairs.join(","))
        }
    }
}

#[cfg(test)]
mod tests;

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
}

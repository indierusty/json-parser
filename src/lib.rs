use std::{collections::HashMap, iter::Peekable, str::Chars};

// TODO: report error with specific message with custom error type

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn parse(src: &str) -> Result<Value, ()> {
    let mut src = src.chars().peekable();
    object(&mut src)
}

fn value(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    match src.peek() {
        Some('{') => object(src),
        Some('"') => string(src),
        Some('[') => array(src),
        Some(c) if c.is_numeric() => number(src),
        _ => bool_or_null(src),
    }
}

fn object(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    // consume '{'
    skip_whitespace(src);
    expect(src, '{')?;
    skip_whitespace(src);

    if src.next_if_eq(&'}').is_some() {
        // empty object
        return Ok(Value::Object(HashMap::new()));
    }

    let mut object = HashMap::new();

    loop {
        skip_whitespace(src);

        let key = string(src)?;

        skip_whitespace(src);
        expect(src, ':')?;
        skip_whitespace(src);

        let value = value(src)?;

        if let Value::String(s) = key {
            object.insert(s, value);
        }

        if let Some(_) = src.next_if_eq(&',') {
            continue;
        } else {
            break;
        }
    }

    // consume '}'
    skip_whitespace(src);
    expect(src, '}')?;

    Ok(Value::Object(object))
}

fn string(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    skip_whitespace(src);
    // consume '"'
    expect(src, '"')?;

    let mut res = String::new();

    while let Some(c) = src.next_if(|c| *c != '"') {
        res.push(c);
    }

    // consume '"'
    expect(src, '"')?;
    Ok(Value::String(res))
}

fn array(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    // consume '['
    skip_whitespace(src);
    expect(src, '[')?;
    skip_whitespace(src);

    if src.next_if_eq(&']').is_some() {
        // empty array
        return Ok(Value::Array(Vec::new()));
    }

    let mut array = Vec::new();

    loop {
        skip_whitespace(src);

        let value = value(src)?;
        array.push(value);

        skip_whitespace(src);
        if let Some(_) = src.next_if_eq(&',') {
            continue;
        } else {
            break;
        }
    }

    // consume ']'
    skip_whitespace(src);
    expect(src, ']')?;

    Ok(Value::Array(array))
}

fn number(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    skip_whitespace(src);

    let mut buf = String::new();

    while let Some(c) = src.next_if(|c| c.is_numeric()) {
        buf.push(c);
    }

    // TODO: handle number parsing error
    Ok(Value::Number(buf.parse().expect("parse number")))
}

fn bool_or_null(src: &mut Peekable<Chars>) -> Result<Value, ()> {
    skip_whitespace(src);

    let mut buf = String::new();

    while let Some(c) = src.next_if(|c| *c != ',') {
        buf.push(c);
    }

    match buf.as_str() {
        "true" => Ok(Value::True),
        "false" => Ok(Value::False),
        "null" => Ok(Value::Null),
        _ => {
            eprintln!("unexpected value {}", buf);
            // TODO: make new error type
            Err(())
        }
    }
}

fn skip_whitespace(src: &mut Peekable<Chars>) {
    while let Some(&c) = src.peek() {
        if c.is_whitespace() {
            src.next();
        } else {
            break;
        }
    }
}

fn expect(src: &mut Peekable<Chars>, expected: char) -> Result<(), ()> {
    if src.next_if_eq(&expected).is_none() {
        eprintln!("expected '{}' found '{:?}'", expected, src.peek());
        Err(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let left = r#"{"code": 200,
            "success": true,
            "notrust": null,
            "payload": {
                "features": [
                    "recursive",
                    "easy",
                    "fun"
                ]
            }
        }"#;

        let left = parse(left);

        let mut body = HashMap::new();
        let mut payload = HashMap::new();
        let features = vec![
            Value::String("recursive".to_string()),
            Value::String("easy".to_string()),
            Value::String("fun".to_string()),
        ];

        payload.insert("features".to_string(), Value::Array(features));

        body.insert("code".to_string(), Value::Number(200.0));
        body.insert("notrust".to_string(), Value::Null);
        body.insert("success".to_string(), Value::True);
        body.insert("payload".to_string(), Value::Object(payload));

        let right = Ok(Value::Object(body));

        assert_eq!(left, right);
    }

    #[test]
    fn empty() {
        let l = parse("{}");
        let r = Value::Object(HashMap::new());

        assert_eq!(l, Ok(r));

        let l = parse("{\"name\": []}");
        let mut object = HashMap::new();
        object.insert("name".to_string(), Value::Array(Vec::new()));
        let r = Value::Object(object);

        assert_eq!(l, Ok(r));
    }
}

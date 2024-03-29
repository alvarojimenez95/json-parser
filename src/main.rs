use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let mut display = false;
    let mut file_path = "".to_owned();

    for i in 1..args.len() {
        if args[i] == "--display" || args[i] == "-d" {
            display = true;
        } else if args[i] == "--file" || args[i] == "-f" {
            file_path = args[i + 1].clone();
        }
    }

    let contents = fs::read_to_string(file_path).unwrap();
    let parsed = parser(&contents);
    match &parsed {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
    if display {
        println!("{:#?}", parsed);
    }
    let duration = start.elapsed();
    println!("Time elapsed in your function is: {:?}", duration);
}

fn parse_null(input: &str) -> Result<JsonValue, &str> {
    match input {
        "null" => Ok(JsonValue::JsonNull),
        _ => Err("Unable to parse JSON null value."),
    }
}

fn parse_bool(input: &str) -> Result<JsonValue, &str> {
    match input {
        "true" => Ok(JsonValue::JsonBoolValue(true)),
        "false" => Ok(JsonValue::JsonBoolValue(false)),
        _ => return Err("Unable to parse JSON boolean value."),
    }
}

fn parse_string(input: &str) -> Result<JsonValue, &str> {
    if !input.starts_with('"') || !input.ends_with('"') {
        return Err("Unable to parse JSON string.");
    }
    if input.len() == 2 {
        return Ok(JsonValue::JsonString(String::new()));
    }
    match input.chars().nth(0).unwrap() == '"' && input.chars().last().unwrap() == '"' {
        true => Ok(JsonValue::JsonString(String::from(input.replace("\"", "")))),
        false => Err("Unable to parse JSON string."),
    }
}

fn split_by_outer_separator<'i>(input: &'i str, separator: &char) -> Vec<&'i str> {
    let mut result = vec![];
    let mut start = 0;
    let mut open_brackets = 0;
    let mut inside_quotes = false;

    for (index, character) in input.char_indices() {
        match character {
            '"' => inside_quotes = !inside_quotes,
            '[' | '{' => {
                if !inside_quotes {
                    open_brackets += 1
                }
            }
            ']' | '}' => {
                if !inside_quotes {
                    open_brackets -= 1
                }
            }
            c if &c == separator => {
                if !inside_quotes && open_brackets == 0 {
                    result.push(&input[start..index]);
                    start = index + 1;
                }
            }
            _ => (),
        }
    }

    result.push(&input[start..]);
    result
}

fn parse_array(input: &str) -> Result<JsonValue, &str> {
    let mut values = vec![];
    // trim the new lines and whitespaces
    let input = &input.trim().replace("\n", "").replace("\t", "");
    if !input.starts_with('[') || !input.ends_with(']') {
        return Err("Unable to parse JSON array.");
    }
    if input.len() == 2 {
        return Ok(JsonValue::JsonArray(vec![]));
    }
    let input = &input[1..input.len() - 1];
    let input_values: Vec<&str> = split_by_outer_separator(input, &','); // split by elements by comma

    for value in input_values {
        match parser(value) {
            Ok(parsed_value) => values.push(parsed_value),
            _ => return Err("Unable to parse JSON array."),
        }
    }
    Ok(JsonValue::JsonArray(values))
}

fn trim_whitespaces_except_quoted(input: &str) -> String {
    let mut in_quote = false;
    let mut result = String::new();

    for c in input.chars() {
        if c == '"' {
            in_quote = !in_quote;
        }

        if !c.is_whitespace() || in_quote {
            result.push(c);
        }
    }

    result
}

fn parse_object(input: &str) -> Result<JsonValue, &str> {
    let mut values = HashMap::new();
    // trim and remove whitespaces
    let input = trim_whitespaces_except_quoted(&input.trim().replace("\n", "").replace("\t", ""));
    if !input.starts_with('{') || !input.ends_with('}') {
        return Err("Unable to parse JSON object.");
    }
    if input.len() == 2 {
        return Ok(JsonValue::JsonObject(HashMap::new()));
    }
    let input = &input[1..input.len() - 1];
    let input_values: Vec<&str> = split_by_outer_separator(input, &','); // separate the final commas
    for value in input_values {
        let split: Vec<&str> = split_by_outer_separator(value, &':'); // separate key/value pair
        if split.len() != 2 {
            return Err("Unable to parse JSON object.");
        }
        let key = match parse_string(split[0]) {
            Ok(JsonValue::JsonString(s)) => JsonObjectKey::new(&s.replace("\"", "")),
            _ => unreachable!(),
        };
        match parser(split[1]) {
            Ok(parsed_value) => {
                values.insert(key, parsed_value);
            }
            Err(_) => return Err("Unable to parse JSON object."),
        }
    }

    Ok(JsonValue::JsonObject(values))
}

fn parse_number(input: &str) -> Result<JsonValue, &str> {
    let parsed = input.parse::<f64>();
    match parsed {
        Ok(result) => Ok(JsonValue::JsonNumber(result)),
        Err(_) => return Err("Unable to parse number"),
    }
}

fn parser(input: &str) -> Result<JsonValue, &str> {
    if let Ok(json_bool) = parse_bool(input) {
        return Ok(json_bool);
    } else {
    }
    if let Ok(json_null) = parse_null(input) {
        return Ok(json_null);
    } else {
    }
    if let Ok(json_string) = parse_string(input) {
        return Ok(json_string);
    } else {
    }
    if let Ok(json_array) = parse_array(input) {
        return Ok(json_array);
    }
    if let Ok(json_object) = parse_object(input) {
        return Ok(json_object);
    }
    if let Ok(json_number) = parse_number(input) {
        return Ok(json_number);
    } else {
        println!("{} error \n", input);
    }
    return Err("Unable to parse JSON file");
}

#[derive(Debug)]
enum JsonValue {
    JsonString(String),
    JsonNull,
    JsonNumber(f64),
    JsonBoolValue(bool),
    JsonArray(Vec<JsonValue>),
    JsonObject(HashMap<JsonObjectKey, JsonValue>),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct JsonObjectKey {
    key: String,
}

impl JsonObjectKey {
    fn new(key: &str) -> Self {
        JsonObjectKey {
            key: key.to_owned(),
        }
    }
}

#[test]
fn test_parser_valid_input_2() {
    let contents = fs::read_to_string("./test_01.json").unwrap();
    let parsed = parser(&contents);
    assert!(parsed.is_ok());
}

#[test]
fn test_parser_valid_input_3() {
    let contents = fs::read_to_string("./test_02.json").unwrap();
    let parsed = parser(&contents);
    assert!(parsed.is_ok());
}

#[test]
fn test_parser_valid_input_1() {
    let contents = fs::read_to_string("./test_01.json").unwrap();
    let parsed = parser(&contents);
    assert!(parsed.is_ok());
    let data = parsed.unwrap();

    // Check that the parsed value is a JsonArray
    let array = match data {
        JsonValue::JsonArray(array) => array,
        _ => panic!("Expected JsonArray"),
    };

    // Access the first element of the array
    let first_element = &array[0];

    // Check that the first element is a JsonObject
    let object = match first_element {
        JsonValue::JsonObject(object) => object,
        _ => panic!("Expected JsonObject"),
    };

    // Look up the value of the key "billTo" in the object
    let bill_to = object.get(&JsonObjectKey {
        key: "billTo".to_owned(),
    });

    // Check that the value exists and is the correct type
    let bill_to_value = match bill_to {
        Some(JsonValue::JsonObject(bill_to_value)) => bill_to_value,
        _ => panic!("Expected JsonObject for key 'billTo'"),
    };

    // Access the value of a zip key
    let zip = bill_to_value.get(&JsonObjectKey {
        key: "zip".to_owned(),
    });

    // Check that the value exists and is the correct type
    let zip_value = match zip {
        Some(JsonValue::JsonString(zip_value)) => zip_value,
        _ => panic!("Expected JsonString for key 'zip'"),
    };
    // Assert that the value of "zip" is "98999"
    assert_eq!(zip_value, "12345");
}

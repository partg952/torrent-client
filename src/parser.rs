use std::collections::BTreeMap;
use std::str::from_utf8;

use enum_as_inner::EnumAsInner;

//d4:name12:hello.txt12:piece lengthi16384e6:lengthi13e6:pieces20:<20 bytes>e
enum InferredBencode {
    BenString,
    BenInt,
    BenList,
    BenDict,
    Invalid,
}
#[derive(Debug, Clone,EnumAsInner)]
pub enum BencodeType {
    BenString(Vec<u8>),
    BenInt(u32),
    BenList(Vec<BencodeType>),
    BenDict(BTreeMap<Vec<u8>, BencodeType>),
    InvalidType,
}
pub fn process_string(size: u16, cursor: &mut usize, text: &[u8]) -> Vec<u8> {
    let mut collect_chars = Vec::new();
    for _ in 0..size {
        if *cursor >= text.len() {
            break;
        }
        collect_chars.push(text[*cursor]);
        *cursor = *cursor + 1;
    }
    return collect_chars;
}
pub fn process_length(cursor: &mut usize, text: &[u8]) -> u16 {
    let mut collected_chars: Vec<u8> = Vec::new();
    while *cursor < text.len() {
        if text[*cursor] == b':' {
            *cursor = *cursor + 1;
            break;
        } else {
            collected_chars.push(text[*cursor]);
            *cursor = *cursor + 1;
        }
    }
    let number_string = match from_utf8(&collected_chars) {
        Ok(value) => value,
        Err(_) => return 0,
    };
    let final_length: u16 = number_string.parse().unwrap_or(0);
    return final_length;
}

pub fn process_integer(cursor: &mut usize, text: &[u8]) -> u32 {
    if *cursor >= text.len() {
        return 0;
    }
    *cursor += 1;
    let mut collect_chars = Vec::new();
    while *cursor < text.len() && text[*cursor] != b'e' {
        collect_chars.push(text[*cursor]);
        *cursor = *cursor + 1;
    }
    let number_string = match from_utf8(&collect_chars) {
        Ok(value) => value,
        Err(_) => return 0,
    };
    let final_integer: u32 = number_string.parse().unwrap_or(0);
    if *cursor < text.len() && text[*cursor] == b'e' {
        *cursor += 1;
    }
    return final_integer;
}
fn infer_type(byte: u8) -> InferredBencode {
    if byte == b'i' {
        return InferredBencode::BenInt;
    } else if byte >= b'0' && byte <= b'9' {
        return InferredBencode::BenString;
    } else if byte == b'l' {
        return InferredBencode::BenList;
    } else if byte == b'd' {
        return InferredBencode::BenDict;
    } else {
        return InferredBencode::Invalid;
    }
}
pub fn parse_value(cursor: &mut usize, text: &[u8]) -> BencodeType {
    if *cursor >= text.len() {
        return BencodeType::InvalidType;
    }
    let inferred_type = infer_type(text[*cursor]);
    match inferred_type {
        InferredBencode::BenInt => {
            let integer_result = process_integer(cursor, text);
            return BencodeType::BenInt(integer_result);
        }
        InferredBencode::BenString => {
            let length = process_length(cursor, text);
            let string_result = process_string(length, cursor, text);
            return BencodeType::BenString(string_result);
        }
        InferredBencode::BenList => {
            let list = process_list(cursor, text);
            return BencodeType::BenList(list);
        }
        InferredBencode::BenDict => {
            let dict = process_dict(cursor, text);
            return BencodeType::BenDict(dict);
        }
        InferredBencode::Invalid => {
            return BencodeType::InvalidType;
        }
    }
}
fn process_list(cursor: &mut usize, text: &[u8]) -> Vec<BencodeType> {
    let mut list: Vec<BencodeType> = Vec::new();
    if *cursor >= text.len() {
        return list;
    }
    *cursor += 1;
    while *cursor < text.len() && text[*cursor] != b'e' {
        let prev_cursor = *cursor;
        let value = parse_value(cursor, text);
        if matches!(value, BencodeType::InvalidType) || *cursor == prev_cursor {
            break;
        }
        list.push(value);
    }
    if *cursor < text.len() && text[*cursor] == b'e' {
        *cursor += 1;
    }
    return list;
}
fn process_dict(cursor: &mut usize, text: &[u8]) -> BTreeMap<Vec<u8>, BencodeType> {
    if *cursor >= text.len() {
        return BTreeMap::new();
    }
    *cursor += 1;
    let mut dict = BTreeMap::new();
    while *cursor < text.len() && text[*cursor] != b'e' {
        let prev_cursor = *cursor;
        let string = parse_value(cursor, text);
        if let BencodeType::BenString(key) = string {
            let value = parse_value(cursor, text);
            if matches!(value, BencodeType::InvalidType) {
                break;
            }
            dict.insert(key, value);
        } else {
            break;
        }
        if *cursor == prev_cursor {
            break;
        }
    }
    if *cursor < text.len() && text[*cursor] == b'e' {
        *cursor += 1;
    }
    return dict;
}
pub fn extract_dict_field_value(
    parsed_value: BencodeType,
    field_name: &[u8],
) -> (BencodeType, bool) {
    if let BencodeType::BenDict(parsed_map) = parsed_value {
        if parsed_map.contains_key(field_name) {
            return (parsed_map[field_name].clone(), true);
        }
        for value in parsed_map.values() {
            match value {
                BencodeType::BenDict(_) => {
                    let search_response = extract_dict_field_value(value.clone(), field_name);
                    if search_response.1 {
                        return search_response;
                    }
                },
                _=> {}
            }
        }
    }
    return (BencodeType::InvalidType, false);
}

use std::collections::BTreeMap;
use std::str::from_utf8;

use crate::parser;

//d4:name12:hello.txt12:piece lengthi16384e6:lengthi13e6:pieces20:<20 bytes>e
pub enum InferredBencode {
    BenString,
    BenInt,
    BenList,
    BenDict,
}
#[derive(Debug)]
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
        collect_chars.push(text[*cursor]);
        *cursor = *cursor + 1;
    }
    return collect_chars;
}
pub fn process_length(cursor: &mut usize, text: &[u8]) -> u16 {
    let mut collected_chars: Vec<u8> = Vec::new();
    loop {
        if text[*cursor] == b':' {
            *cursor = *cursor + 1;
            break;
        } else {
            collected_chars.push(text[*cursor]);
            *cursor = *cursor + 1;
        }
    }
    let mut number_string = from_utf8(&collected_chars).unwrap();
    let final_length: u16 = number_string
        .parse()
        .expect(format!("Error occured at : {}", number_string).as_str());
    return final_length;
}

pub fn process_integer(cursor: &mut usize, text: &[u8]) -> u32 {
    *cursor += 1;
    let mut collect_chars = Vec::new();
    while text[*cursor] != b'e' {
        collect_chars.push(text[*cursor]);
        *cursor = *cursor + 1;
    }
    let mut number_string = from_utf8(&collect_chars).unwrap();
    println!("{}", number_string);
    let final_integer: u32 = number_string.parse().unwrap();
    *cursor += 1;
    return final_integer;
}
pub fn infer_type(byte: u8) -> InferredBencode {
    if byte == b'i' {
        return InferredBencode::BenInt;
    } else if byte >= b'0' && byte <= b'9' {
        return InferredBencode::BenString;
    } else if byte == b'l' {
        return InferredBencode::BenList;
    } else {
        return InferredBencode::BenDict;
    }
}
pub fn parse_value(cursor: &mut usize, text: &[u8]) -> BencodeType {
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
        _ => {
            return BencodeType::InvalidType;
        }
    }
}
pub fn process_list(cursor: &mut usize, text: &[u8]) -> Vec<BencodeType> {
    let mut list: Vec<BencodeType> = Vec::new();
    *cursor += 1;
    while text[*cursor] != b'e' {
        let value = parse_value(cursor, text);
        list.push(value);
    }
    *cursor += 1;
    return list;
}
pub fn process_dict(cursor: &mut usize, text: &[u8]) -> BTreeMap<Vec<u8>, BencodeType> {
    *cursor += 1;
    let mut dict = BTreeMap::new();
    while text[*cursor] != b'e' {
        let string = parse_value(cursor, text);
        if let BencodeType::BenString(key) = string {
            let value = parse_value(cursor, text);
            let key_clone = key.clone();
            dict.insert(key, value);
        }
    }
    *cursor += 1;
    return dict;
}
pub fn extract_dict_field_value(parsed_value : BencodeType , field_name : &[u8]) -> BencodeType {
    if let BencodeType::BenDict(parsed_map) = parsed_value {
        if ()
    }
    return;
}
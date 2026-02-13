use std::fs::read;
use sha1::Digest;
use crate::parser;

pub fn read_info(text: &[u8]) -> Option<(usize, usize)> {
    let mut depth = 0;
    let mut start = 0;
    let mut end = 0;
    let mut i = 0;
    while i < text.len() - 6 {
        if &text[i..i + 6] == b"4:info" {
            let d_index = i + 6;
            if d_index < text.len() && text[d_index] == b'd' {
                start = d_index;
            }
            break;
        }
        i += 1;
    }
    i = start;
    while true {
        let byte = text[i];
        if byte >= b'0' && byte <= b'9' {
            let length = parser::process_length(&mut i, text);
            let _ = parser::process_string(length, &mut i, text);
            continue;
        }
        if byte == b'i' {
            let _ = parser::process_integer(&mut i, text);
            continue;
        }
        if byte == b'd' || byte == b'l' {
            depth += 1;
        }
        if byte == b'e' {
            depth -= 1;
            if depth == 0 {
                end = i;
                break;
            }
        }
        i += 1;
    }
    Some((start, end))
}

pub fn hash_info(text : &[u8]) -> Vec<u8> {
    let start_and_end = read_info(text);
    let mut hash_vector = Vec::new();
    if let Some((start , end)) = start_and_end {
        let mut hasher = sha1::Sha1::new();
        hasher.update(&text[start..=end]);
        let final_hash = hasher.finalize();
        hash_vector =  final_hash.to_vec();
    }
    return hash_vector;
}
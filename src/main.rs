use std::str::from_utf8;
use rand::{Rng, RngCore};
use crate::hasher::hash_info;
use crate::parser::{parse_value, process_integer, process_length, process_list, process_string};
use std::fs;
mod parser;
mod hasher;
mod network;

fn generate_peerid() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8;20];
    rng.fill_bytes(&mut random_bytes);
    return random_bytes.to_vec();
}
///peer_id
///info_hash
///left
/// compact=1
/// downloaded=1
///uploaded=1

fn generate_tracker_url(peer_id:Vec<u8> ,info_hash : Vec<u8> , left : u8) -> String {
    
    return;
}
fn main() {
    let mut cursor = 0 as usize;
    let peer_id = generate_peerid();
    let hash_info = hash_info(text);
    let file_contents = fs::read("sample.torrent").unwrap();

    println!("{:?}", parse_value(&mut cursor, &file_contents));
}


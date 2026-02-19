use crate::hasher::hash_info;
use crate::parser::{
    extract_dict_field_value, parse_value, process_integer, process_length, process_string,
};
use enum_as_inner::EnumAsInner;
use rand::{Rng, RngCore, random};
use std::fmt::format;
use std::fs;
use std::net::{ToSocketAddrs, UdpSocket};
use std::str::from_utf8;
mod hasher;
mod network;
mod parser;

fn generate_peerid() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 20];
    rng.fill_bytes(&mut random_bytes);
    return random_bytes.to_vec();
}
fn generate_transaction_id() -> u64 {
    let mut rng = rand::rng();
    let random_number: u64 = rng.random();
    return random_number;
}
///peer_id
///info_hash
///left
/// compact=1
/// downloaded=1
///uploaded=1
fn main() {
    let mut cursor = 0 as usize;
    let peer_id = generate_peerid();
    let file_contents = fs::read("sample.torrent").unwrap();
    let parsed_contents = parse_value(&mut cursor, &file_contents);
    let announce_string = (extract_dict_field_value(parsed_contents, "announce".as_bytes()).0)
        .into_ben_string()
        .unwrap();


    let announce_url = &from_utf8(&announce_string).unwrap()[6..];
    println!("{}", announce_url);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    //connecting to the tracker
    let transaction_id = generate_transaction_id();

    let mut connection_request = Vec::with_capacity(16);
    connection_request.extend_from_slice(&0x41727101980u64.to_be_bytes());
    connection_request.extend_from_slice(&0u32.to_be_bytes());
    connection_request.extend_from_slice(&transaction_id.to_be_bytes());

    let tracker_host = announce_url
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    socket.send_to(&connection_request, tracker_host).unwrap();
    let mut response_buffer = [0u8;16];
    socket.recv_from(&mut response_buffer).unwrap();
    println!("{:?}" , response_buffer);
}

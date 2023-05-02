use std::collections::HashMap;

use anyhow::Result;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::buffer_writer::BufferWriter;
mod buffer_reader;
mod buffer_writer;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let mut opts: Vec<(&str, &str)> = Vec::new();
    opts.push(("user", "filipton"));
    opts.push(("database", "neondb"));
    opts.push(("client_encoding", "UTF8"));

    let mut writer = buffer_writer::BufferWriter::new();
    writer.write_i16(3).write_i16(0);

    for (key, value) in opts {
        writer.write_cstring(&key).write_cstring(&value);
    }

    let mut password_writer = buffer_writer::BufferWriter::new();
    let password = password_writer
        .write_cstring("9l2FcxtusEYC")
        .flush(Some(b'p'));

    println!("{:?}", hex::encode_upper(&password));

    let length = writer.get_length();
    let body = writer.write_cstring("").add(&password).flush(None);

    let res = BufferWriter::new()
        .write_i32(length as i32)
        .add(&body)
        .flush(None);

    println!("{:?}", hex::encode_upper(res));
    return Ok(());

    let mut ws_client =
        ClientBuilder::new("wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2")
            .unwrap()
            .connect_secure(None)
            .unwrap();

    let login_query = generate_login("filipton", "9l2FcxtusEYC", "neondb");
    let generate_query = generate_query("SELECT * FROM tests");
    println!("{:?}", hex::encode(&generate_query));

    ws_client
        .send_message(&Message::binary(login_query))
        .unwrap();
    for _ in 0..2 {
        _ = ws_client.recv_message().unwrap();
    }

    ws_client
        .send_message(&Message::binary(generate_query))
        .unwrap();
    let msg = ws_client.recv_message().unwrap();
    if let OwnedMessage::Binary(bytes) = msg {
        let mut parser = parser::Parser::new(&bytes);
        _ = parser.parse();
    }

    Ok(())
}

// TODO: make serialzier for this LMAOOOO
const START_HEX: &'static [u8; 7] = &[0, 0, 0, 0x3C, 0, 0x03, 0];
const USER_HEX: &'static [u8; 6] = b"\x00user\x00";
const DATABASE_HEX: &'static [u8; 10] = b"\x00database\x00";
const CLIENT_ENCODING_HEX: &'static [u8; 17] = b"\x00client_encoding\x00";
const UTF8_HEX: &'static [u8; 5] = b"UTF8\x00";
const PASSWORD_HEX: &'static [u8; 2] = b"\x00p";
fn generate_login(username: &str, password: &str, db: &str) -> Vec<u8> {
    let mut tmp = Vec::new();
    tmp.extend_from_slice(START_HEX);
    tmp.extend_from_slice(USER_HEX);
    tmp.extend_from_slice(username.as_bytes());
    tmp.extend_from_slice(DATABASE_HEX);
    tmp.extend_from_slice(db.as_bytes());
    tmp.extend_from_slice(CLIENT_ENCODING_HEX);
    tmp.extend_from_slice(UTF8_HEX);
    tmp.extend_from_slice(PASSWORD_HEX);

    let password_len_be: [u8; 4] = (password.len() as u32 + 5).to_be_bytes();

    tmp.extend_from_slice(&password_len_be);
    tmp.extend_from_slice(password.as_bytes());
    tmp.push(0);
    tmp
}

const QUERY_HEX: &'static [u8; 1] = b"Q";
fn generate_query(query: &str) -> Vec<u8> {
    let mut tmp = Vec::new();
    tmp.extend_from_slice(QUERY_HEX);

    let query_len_be: [u8; 4] = (query.len() as u32 + 5).to_be_bytes();

    tmp.extend_from_slice(&query_len_be);
    tmp.extend_from_slice(query.as_bytes());
    tmp.push(0);
    tmp
}

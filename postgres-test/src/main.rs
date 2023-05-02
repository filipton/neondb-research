use std::collections::HashMap;

use anyhow::Result;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::buffer_writer::BufferWriter;
mod buffer_reader;
mod buffer_writer;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ws_client =
        ClientBuilder::new("wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2")
            .unwrap()
            .connect_secure(None)
            .unwrap();

    let login_query = generate_login("filipton", "9l2FcxtusEYC", "neondb");
    let generate_query = generate_query("SELECT * FROM tests");

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

fn generate_login(username: &str, password: &str, db: &str) -> Vec<u8> {
    let mut opts: Vec<(&str, &str)> = Vec::new();
    opts.push(("user", username));
    opts.push(("database", db));
    opts.push(("client_encoding", "UTF8"));

    let mut writer = buffer_writer::BufferWriter::new();
    writer.write_i16(3).write_i16(0);

    for (key, value) in opts {
        writer.write_cstring(&key).write_cstring(&value);
    }

    let mut password_writer = buffer_writer::BufferWriter::new();
    let password = password_writer.write_cstring(password).flush(Some(b'p'));

    let length = writer.get_length();
    let body = writer.write_cstring("").add(&password).flush(None);

    let res = BufferWriter::new()
        .write_i32(length as i32)
        .add(&body)
        .flush(None);

    res
}

fn generate_query(query: &str) -> Vec<u8> {
    let mut writer = buffer_writer::BufferWriter::new();
    writer.write_cstring(query);
    writer.flush(Some(b'Q'))
}

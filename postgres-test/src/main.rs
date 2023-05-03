use std::net::TcpStream;

use crate::buffer_writer::BufferWriter;
use anyhow::Result;
use websocket::{native_tls::TlsStream, sync::Client, ClientBuilder, Message, OwnedMessage};

mod buffer_reader;
mod buffer_writer;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ws_client = connect(
        "filipton",
        "9l2FcxtusEYC",
        "neondb",
        "wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2",
    );

    let query = generate_query("SELECT * FROM tests");
    let query2 = generate_query("INSERT INTO tests (id, val, tex) VALUES (31, 59455, 'vcxvxc')");
    let query3 = generate_query("DELETE FROM tests WHERE id = 31");

    ws_client
        .send_message(&Message::binary(query.clone()))
        .unwrap();

    ws_client.send_message(&Message::binary(query2)).unwrap();
    ws_client
        .send_message(&Message::binary(query.clone()))
        .unwrap();
    ws_client.send_message(&Message::binary(query3)).unwrap();
    ws_client.send_message(&Message::binary(query)).unwrap();

    for _ in 0..5 {
        let msg = ws_client.recv_message().unwrap();
        if let OwnedMessage::Binary(bytes) = msg {
            let mut parser = parser::Parser::new(&bytes);
            _ = parser.parse();
        }
        println!("------------------");
    }

    Ok(())
}

fn connect(username: &str, password: &str, db: &str, ws_url: &str) -> Client<TlsStream<TcpStream>> {
    let mut ws_client = ClientBuilder::new(ws_url)
        .unwrap()
        .connect_secure(None)
        .unwrap();

    let login_query = generate_login(username, password, db);
    ws_client
        .send_message(&Message::binary(login_query))
        .unwrap();

    for _ in 0..2 {
        let msg = ws_client.recv_message().unwrap();
        if let OwnedMessage::Binary(bytes) = msg {
            let mut parser = parser::Parser::new(&bytes);
            _ = parser.parse();
        }
    }

    ws_client
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

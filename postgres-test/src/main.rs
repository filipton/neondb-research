use crate::buffer_writer::BufferWriter;
use anyhow::Result;

mod buffer_reader;
mod buffer_writer;
mod parser;

/*
pub enum Response {
    DataRow(...),
    ...
}
*/
// AND THEN WHILE MAKING RESPONSE, WE ARE GETTING VECTOR OF RESPONSES
// GET TILL "READY FOR QUERY"

fn main() -> Result<()> {
    let task1 = wasm_rs_async_executor::single_threaded::spawn(async move {
        let (mut ws, _wsio) = ws_stream_wasm::WsMeta::connect(
            "wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2",
            None,
        )
        .await
        .unwrap();

        println!("Connected to server");
    });

    wasm_rs_async_executor::single_threaded::run(Some(task1.task()));

    /*
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
    _ = ws_client.set_nonblocking(true);

    for _ in 0..15 {
        let msg = ws_client.recv_message();
        if let Ok(msg) = msg {
            if let OwnedMessage::Binary(bytes) = msg {
                let mut parser = parser::Parser::new(&bytes);
                _ = parser.parse();
            }
            println!("------------------");
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    */

    Ok(())
}

/*
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
*/

fn generate_login(username: &str, password: &str, db: &str) -> Vec<u8> {
    let mut startup = BufferWriter::new();
    startup
        .write_i16(3)
        .write_i16(0)
        .write_cstring("user")
        .write_cstring(username)
        .write_cstring("database")
        .write_cstring(db)
        .write_cstring("client_encoding")
        .write_cstring("UTF8");
    let startup_length = startup.get_length();

    let password = BufferWriter::new()
        .write_cstring(password)
        .flush(Some(b'p'));

    let startup_password = startup.write_cstring("").add(&password).flush(None);
    let res = BufferWriter::new()
        .write_i32(startup_length as i32)
        .add(&startup_password)
        .flush(None);

    res
}

fn generate_query(query: &str) -> Vec<u8> {
    let mut writer = buffer_writer::BufferWriter::new();
    writer.write_cstring(query);
    writer.flush(Some(b'Q'))
}

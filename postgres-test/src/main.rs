fn main() {
    let login_query = generate_login("filipton", "9l2FcxtusEYC", "neondb");
    let hex = hex::encode(&login_query);
    println!("{}", hex);

    let generate_query = generate_query("SELECT now()");
    let hex = hex::encode(&generate_query);
    println!("{}", hex);
}

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

    let password_be = (password.len() + 5).to_be_bytes();
    let password_be = &password_be[(password_be.len() - 4)..];

    tmp.extend_from_slice(password_be);
    tmp.extend_from_slice(password.as_bytes());
    tmp.push(0);

    tmp
}

const QUERY_HEX: &'static [u8; 1] = b"Q";
fn generate_query(query: &str) -> Vec<u8> {
    let mut tmp = Vec::new();
    tmp.extend_from_slice(QUERY_HEX);

    tmp
}

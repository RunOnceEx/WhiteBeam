// Database
use crate::common::db;
// Public key encryption and signatures
use crate::common::crypto;

fn set_console_secret(secret: &str, expiry: &str) -> String {
    let conn: rusqlite::Connection = db::db_open();
    db::update_config(&conn, "console_secret", secret);
    db::update_config(&conn, "console_secret_expiry", expiry);
    String::from("OK")
}

fn invalid_request() -> String {
    // Avoid providing a cryptographic oracle
    String::from("OK")
}

// GET /service/public_key
pub async fn public_key() -> Result<impl warp::Reply, warp::Rejection> {
    match crypto::get_client_public_key() {
            Ok(client_public_key) => Ok(hex::encode(client_public_key)),
            Err(_e) => return Err(warp::reject::not_found())
    }
}

// POST /service/encrypted
pub fn encrypted(crypto_box_object: crypto::CryptoBox) -> impl warp::Reply {
    let server_message = match crypto::process_request(crypto_box_object) {
        Ok(server_msg) => server_msg,
        Err(_e) => return invalid_request()
    };
    match server_message.action.as_ref() {
        "set_console_secret" => set_console_secret(&server_message.parameters[0],
                                                   &server_message.parameters[1]),
        _ => invalid_request()
    }
}

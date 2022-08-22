extern crate sqlite;
use std::io::{Error, ErrorKind};

pub fn create_tables() {
    let connection = sqlite::open("users.db").unwrap();

    connection.execute("CREATE TABLE IF NOT EXISTS users (username TEXT, e_mail TEXT, password_hash TEXT);",).unwrap();
}

pub fn find_user_object(username: &str) -> Result<(String, String, String), Error> {
    let connection = sqlite::open("users.db").unwrap();

    let content = format!("SELECT * FROM users WHERE username = '{}';", username);
    let mut statement = connection.prepare(content).unwrap();
    
    if let sqlite::State::Row = statement.next().unwrap() {

        let obj_username = statement.read::<String>(0).unwrap();
        let obj_e_mail = statement.read::<String>(1).unwrap();
        let obj_password_hash = statement.read::<String>(2).unwrap();

        return Ok((obj_username, obj_password_hash, obj_e_mail));

    } else {
        Err(Error::new(ErrorKind::Other, "username dosn't exists"))
    }
}

pub fn save_user_object(username: &str, e_mail: &str, password_hash: &str) {
    let connection = sqlite::open("users.db").unwrap();
    
    let content = format!("INSERT INTO users VALUES('{}', '{}', '{}');", username, e_mail, password_hash);
    connection.execute(content).unwrap();
}

pub fn check_username(username: &str) -> bool {
    let connection = sqlite::open("users.db").unwrap();

    let content = format!("SELECT * FROM users WHERE username = '{}';", username);
    let mut statement = connection.prepare(content).unwrap();

    if let sqlite::State::Row = statement.next().unwrap() {
        return true;
    } else {
        return false;
    }
}

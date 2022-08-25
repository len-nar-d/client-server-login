use colored::Colorize;
use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Error, ErrorKind, BufReader};
use std::str::from_utf8;
use serde_derive::Deserialize;
use crate::user;

static CREATE_REQUEST: [u8; 4] = [0,0,0,1];
static LOGIN_REQUEST: [u8; 4] = [0,0,1,0];
static ERROR_MESSAGE: [u8; 4] = [1,0,0,0];
static SUCCESS_MESSAGE: [u8; 4] = [0,0,0,0];


#[derive(Deserialize, Debug)]
struct Settings {
    network: Network,
    securtiy: Security
}

#[derive(Deserialize, Debug)]
struct Network {
    ip_address: String,
    port: String
}

#[derive(Deserialize, Debug)]
struct Security {
    response_time: u64,
    login_trys: u32
}

fn read_settings<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let content = serde_json::from_reader(reader)?;

    Ok(content)
}

pub fn info(msg: String) {
    let message = format!("{} > {}", "[INFO]".yellow(), msg);
    println!("{}", message);
}

pub fn warn(msg: String) {
    let message = format!("{} > {}", "[WARN]".red(), msg);
    println!("{}", message);
}

pub fn login_log_table() {
    let connection = sqlite::open("users.db").unwrap();

    connection.execute("DROP TABLE IF EXISTS login_log").unwrap();
    connection.execute("CREATE TABLE IF NOT EXISTS login_log (ip_address TEXT, success TEXT);",).unwrap();
}

fn log_login(ip_address: &str, success: bool) {
    let connection = sqlite::open("users.db").unwrap();
        
    let content = format!("INSERT INTO login_log VALUES('{}', '{}');", ip_address, success);
    connection.execute(content).unwrap();
}

fn false_attempts_for_ip(ip_address: &str) -> u32 {
    let connection = sqlite::open("users.db").unwrap();
    
    let content = format!("SELECT * FROM login_log WHERE ip_address = '{}' AND success = 'false';", ip_address);
    let mut statement = connection.prepare(content).unwrap();
    
    let mut false_attempts: u32 = 0;
    while let sqlite::State::Row = statement.next().unwrap() {
        false_attempts += 1;
    }

    return false_attempts;
}

fn login_account(mut stream: &TcpStream) -> Result<user::User, Error> {
    let mut username_message = [0 as u8; 64];
    let mut password_message = [0 as u8; 128];

    let username: &str;
    let password: &str;

    match stream.read(&mut username_message) {
        Ok(size) => {
            username = from_utf8(&username_message[0..size]).unwrap();
            if !user::check_username(&username) {
                stream.write(&ERROR_MESSAGE).unwrap();
                return Err(Error::new(ErrorKind::Other, "false username"));
            } else {
                stream.write(&SUCCESS_MESSAGE).unwrap();
            }
        },
        Err(e) => {
            stream.write(&ERROR_MESSAGE).unwrap();
            return Err(e);
        }
    }

    let account = user::User::from_database(&username);

    match stream.read(&mut password_message) {
        Ok(size) => {
            password = from_utf8(&password_message[0..size]).unwrap();
            if !account.password_check(password) {
                stream.write(&ERROR_MESSAGE).unwrap();
                return Err(Error::new(ErrorKind::Other, "false password"));
            } else {
                stream.write(&SUCCESS_MESSAGE).unwrap();
            }
        },
        Err(e) => {
            stream.write(&ERROR_MESSAGE).unwrap();
            return Err(e);
        }
    }

    return Ok(account);
}

fn create_account(mut stream: &TcpStream) -> Result<user::User, Error> {
    let mut username_message = [0 as u8; 64];
    let mut e_mail_message = [0 as u8; 128];
    let mut password_message = [0 as u8; 128];

    let username: &str;
    let e_mail: &str;
    let password: &str;

    match stream.read(&mut username_message) {
        Ok(size) => {
            username = from_utf8(&username_message[0..size]).unwrap();
            if user::check_username(&username) {
                stream.write(&ERROR_MESSAGE).unwrap();
                return Err(Error::new(ErrorKind::Other, "username exists"));
            } else {
                stream.write(&SUCCESS_MESSAGE).unwrap();
            }
        },
        Err(e) => {
            stream.write(&ERROR_MESSAGE).unwrap();
            return Err(e);
        }
    }

    match stream.read(&mut e_mail_message) {
        Ok(size) => {
            e_mail = from_utf8(&e_mail_message[0..size]).unwrap();
            stream.write(&SUCCESS_MESSAGE).unwrap();
        },
        Err(e) => {
            stream.write(&ERROR_MESSAGE).unwrap();
            return Err(e);
        }
    }

    match stream.read(&mut password_message) {
        Ok(size) => {
            password = from_utf8(&password_message[0..size]).unwrap();
            stream.write(&SUCCESS_MESSAGE).unwrap();
        },
        Err(e) => {
            stream.write(&ERROR_MESSAGE).unwrap();
            return Err(e);
        }
    }
    
    let account = user::User::new(&username, &e_mail, &password);

    return Ok(account);
}

fn handle_request(mut stream: TcpStream, ip_address: &str) {
    let mut session_user: Option<user::User> = None;
    let mut data = [0 as u8; 4];
    let settings = read_settings("./src/settings.json").unwrap();
    
    while match stream.read_exact(&mut data) {
        Ok(_) => {
            if false_attempts_for_ip(&ip_address) >= settings.securtiy.login_trys {
                stream.shutdown(Shutdown::Both).unwrap();
            }

            if session_user.is_none() {
                if &LOGIN_REQUEST == &data {
                    match login_account(&stream) {
                        Ok(user) => {
                            info(format!("Login with username: {} | {}", user.get_username(), &ip_address));
                            log_login(&ip_address, true);
                            session_user = Some(user);
                        },
                        Err(e) => {
                            warn(format!("Login error: {} | {}", e, &ip_address));
                            log_login(&ip_address, false);
                        }
                    }
                } else if &CREATE_REQUEST == &data {
                    match create_account(&stream) {
                        Ok(user) => {
                            info(format!("Created account with username: {} | {}", user.get_username(), &ip_address));
                            session_user = Some(user);
                        },
                        Err(e) => warn(format!("Account creation error: {} | {}", e, &ip_address))
                    };
                }
            } else {

                // Space for functions which should only be visible for logged in users
                
            }
            true
        },
        Err(_) => {
            info(format!("Terminating connection with {}", &ip_address));
            false
        }
    } {}

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn run() {
    let settings = read_settings("./src/settings.json").unwrap();
    let server_ip = format!("{}:{}", settings.network.ip_address, settings.network.port);
    let response_time = time::Duration::from_millis(settings.securtiy.response_time);
    
    let listener = TcpListener::bind(server_ip).unwrap();

    info(format!("Listening on port {}", settings.network.port));

    for stream in listener.incoming() {
        let ip_address = stream.as_ref()
            .expect("unexpected Error")
            .peer_addr()
            .unwrap()
            .to_string()
            .split(":")
            .next()
            .unwrap()
            .to_string();

        if false_attempts_for_ip(&ip_address) < settings.securtiy.login_trys {
            match stream {
                Ok(stream) => {
                    info(format!("New connection with {}", &ip_address));
                    thread::spawn(move|| {handle_request(stream, &ip_address)});
                }
                Err(e) => {
                    warn(format!("Error has occured: {}", e));
                }
            }
            thread::sleep(response_time);
        }
    }
    drop(listener);
}


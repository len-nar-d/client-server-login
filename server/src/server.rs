use colored::Colorize;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Error, ErrorKind};
use std::str::from_utf8;
use crate::user;

static CREATE_REQUEST: [u8; 4] = [0,0,0,1];
static LOGIN_REQUEST: [u8; 4] = [0,0,1,0];
static ERROR_MESSAGE: [u8; 4] = [1,0,0,0];
static SUCCESS_MESSAGE: [u8; 4] = [0,0,0,0];


pub fn info(msg: String) {
    let message = format!("{} > {}", "[INFO]".yellow(), msg);
    println!("{}", message);
}

pub fn warn(msg: String) {
    let message = format!("{} > {}", "[WARN]".red(), msg);
    println!("{}", message);
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

fn handle_request(mut stream: TcpStream) {
    let mut session_user: Option<user::User> = None;
    let mut data = [0 as u8; 4];
    
    while match stream.read_exact(&mut data) {
        Ok(_) => {
            if !session_user.is_none(){

                // Space for functions which should only be visible for logged in users
                
            } else {
                if &LOGIN_REQUEST == &data {
                    match login_account(&stream) {
                        Ok(user) => {
                            info(format!("Login with username: {} from {}", user.get_username(), stream.peer_addr().unwrap()));
                            session_user = Some(user);
                        },
                        Err(e) => warn(format!("Login error: {} from {}", e, stream.peer_addr().unwrap()))
                    }
                } else if &CREATE_REQUEST == &data {
                    match create_account(&stream) {
                        Ok(user) => {
                            info(format!("Created account with username: {} from {}", user.get_username(), stream.peer_addr().unwrap()));
                            session_user = Some(user);
                        },
                        Err(e) => warn(format!("Account creation error: {} from", e))
                    };
                }
            }
            true
        },
        Err(_) => {
            info(format!("Terminating connection with {}", stream.peer_addr().unwrap()));
            false
        }
    } {}

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    info("Listening on port 3333".to_string());

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                info(format!("New connection to client with ip: {}", stream.peer_addr().unwrap()));
                thread::spawn(move|| {handle_request(stream)});
            }
            Err(e) => {
                warn(format!("Error has occured: {}", e));
            }
        }
    }
    drop(listener);
}


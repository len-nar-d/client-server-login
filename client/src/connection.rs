use std::net::TcpStream;
use std::io::{Read, Write, Error, ErrorKind, BufReader};
use serde_derive::Deserialize;

static CREATE_REQUEST: [u8; 4] = [0,0,0,1];
static LOGIN_REQUEST: [u8; 4] = [0,0,1,0];
static ERROR_MESSAGE: [u8; 4] = [1,0,0,0];


#[derive(Deserialize, Debug)]
struct Settings {
    server: Server
}

#[derive(Deserialize, Debug)]
struct Server {
    ip_address: String,
    port: String
}

fn read_settings<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let settings = serde_json::from_reader(reader)?;

    Ok(settings)
}

pub fn login_account(username: &str, password: &str, mut stream: &TcpStream) -> Result<(), Error> {
    stream.write(&LOGIN_REQUEST).unwrap();
    stream.write(username.as_bytes()).unwrap();

    let mut answer_username = [0 as u8; 4];
    match stream.read_exact(&mut answer_username) {
        Ok(_) => {
            if &answer_username == &ERROR_MESSAGE {
                return Err(Error::new(ErrorKind::Other, "false username"));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    stream.write(password.as_bytes()).unwrap();

    let mut answer_password = [0 as u8; 4];
    match stream.read_exact(&mut answer_password) {
        Ok(_) => {
            if &answer_password == &ERROR_MESSAGE {
                return Err(Error::new(ErrorKind::Other, "false password"));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    return Ok(());
}

pub fn create_account(username: &str, e_mail: &str, password: &str, mut stream: &TcpStream) -> Result<(), Error> {
    stream.write(&CREATE_REQUEST).unwrap();
    stream.write(username.as_bytes()).unwrap();

    let mut answer_username = [0 as u8; 4];
    match stream.read_exact(&mut answer_username) {
        Ok(_) => {
            if &answer_username == &ERROR_MESSAGE {
                return Err(Error::new(ErrorKind::Other, "username allready exists"));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    stream.write(e_mail.as_bytes()).unwrap();

    let mut answer_e_mail = [0 as u8; 4];
    match stream.read_exact(&mut answer_e_mail) {
        Ok(_) => {
            if &answer_username == &ERROR_MESSAGE {
                return Err(Error::new(ErrorKind::Other, "server didn't read E-mail"));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    stream.write(password.as_bytes()).unwrap();

    let mut answer_password = [0 as u8; 4];
    match stream.read_exact(&mut answer_password) {
        Ok(_) => {
            if &answer_password == &ERROR_MESSAGE {
                return Err(Error::new(ErrorKind::Other, "server didn't read password"));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    return Ok(());
}

pub fn connect() -> Result<TcpStream, Error> {
    let settings = read_settings("./src/settings.json").unwrap();
    let address = format!("{}:{}", settings.server.ip_address, settings.server.port);

    match TcpStream::connect(address) {
        Ok(stream) => {
            return Ok(stream);
        },
        Err(e) => {
            return Err(e);
        }
    }
}


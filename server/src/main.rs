pub mod user;
mod server;

use serde_derive::Deserialize;
use std::io::BufReader;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    network: Network,
    securtiy: Security
}

#[derive(Deserialize, Debug, Clone)]
struct Network {
    ip_address: String,
    port: String
}

#[derive(Deserialize, Debug, Clone)]
struct Security {
    response_time: u64,
    login_trys: u32
}

fn main() {
    user::user_table();
    server::login_log_table();
    println!("\n");
    server::info("Starting server".to_string());
    server::run(read_settings("./src/settings.json").unwrap());
}

fn read_settings<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let content = serde_json::from_reader(reader)?;

    Ok(content)
}
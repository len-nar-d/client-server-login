pub mod database;
pub mod user;
mod server;


fn main() {
    database::create_tables();
    println!("\n");
    server::info("Starting server".to_string());
    server::run();

}


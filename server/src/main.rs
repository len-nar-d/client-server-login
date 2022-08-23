pub mod user;
mod server;


fn main() {
    user::create_table();
    println!("\n");
    server::info("Starting server".to_string());
    server::run();
}


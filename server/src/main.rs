pub mod user;
mod server;


fn main() {
    user::user_table();
    server::login_log_table();
    println!("\n");
    server::info("Starting server".to_string());
    server::run();
}


mod connection;

use colored::Colorize;


fn main() {
    println!("\n{}\n", "[CLIENT STARTING]".green());

    let ip_address = cli_configuration();

    println!("\n{}\n", "[CONNECTING TO SERVER]".green());

    match connection::connect(&ip_address) {
        Ok(stream) => {

            let mut choice = String::new();
            println!("{} or {}:", "[LOGIN]".yellow(), "[REGISTER]".blue());
            std::io::stdin().read_line(&mut choice).unwrap();

            if &choice.trim().to_uppercase() == &"LOGIN".to_string() {
                let data = cli_login();
                match connection::login_account(&data.0, &data.1, &stream) {
                    Ok(_) => {
                        println!("{}", "\n[LOGED IN]\n".green());
                    }
                    Err(e) => {
                        println!("\n{} > {}\n", "[ERROR]".red(), e);
                    }
                }
    
            } else if &choice.trim().to_uppercase() == &"REGISTER".to_string() {
                let data = cli_register();
                match connection::create_account(&data.0, &data.1, &data.2, &stream) {
                    Ok(_) => {
                        println!("{}", "\n[CREATED ACCOUNT]\n".green());
                    }
                    Err(e) => {
                        println!("\n{} > {}\n", "[ERROR]".red(), e);
                    }
                }

            }

            let mut wait = String::new();
            println!("Press enter to terminate connection!");
            std::io::stdin().read_line(&mut wait).unwrap();
            
        },
        Err(e) => println!("\n{} > {}\n", "[ERROR]".red(), e)
    }
}


fn cli_configuration() -> String {
    let mut ip = String::new();
    println!("Please enter server IP: ");
    std::io::stdin().read_line(&mut ip).unwrap();

    let mut port = String::new();
    println!("Please enter port: ");
    std::io::stdin().read_line(&mut port).unwrap();

    return format!("{}:{}", ip.trim(), port.trim());
}

fn cli_login() -> (String, String) {
    let mut username = String::new();
    println!("\nPlease enter username: ");
    std::io::stdin().read_line(&mut username).unwrap();

    let mut password = String::new();
    println!("Please enter password: ");
    std::io::stdin().read_line(&mut password).unwrap();

    return (username.trim().to_string(), password.trim().to_string());
}

fn cli_register() -> (String, String, String) {
    let mut username = String::new();
    println!("\nPlease enter username: ");
    std::io::stdin().read_line(&mut username).unwrap();

    let mut e_mail = String::new();
    println!("Please enter email: ");
    std::io::stdin().read_line(&mut e_mail).unwrap();

    let mut password = String::new();
    println!("Please enter password: ");
    std::io::stdin().read_line(&mut password).unwrap();

    return (username.trim().to_string(), e_mail.trim().to_string(), password.trim().to_string());
}


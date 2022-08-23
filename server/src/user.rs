use sha_crypt::{Sha512Params, sha512_simple, sha512_check};


pub struct User {
    username: String,
    password_hash: String,
    e_mail: String
}

impl User {
    pub fn new(username: &str, e_mail: &str, password: &str) -> Self {
        let password_hash = Self::create_password_hash(password);

        Self::save_user_object(username, e_mail, &password_hash);

        return Self {
            username: username.to_string(),
            password_hash: password_hash,
            e_mail: e_mail.to_string()
        };
    }

    pub fn from_database(username: &str) -> Self {
        let user_data = Self::find_user_object(username);

        return Self {
            username: user_data.0,
            password_hash: user_data.1,
            e_mail: user_data.2
        };
    }

    pub fn password_check(&self, password: &str) -> bool {
        sha512_check(password, &self.password_hash).is_ok()
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_e_mail(&self) -> &String {
        &self.e_mail
    }

    fn create_password_hash(password: &str) -> String {
        let params: Sha512Params = Sha512Params::new(10_000).unwrap();
        sha512_simple(password, &params).unwrap()
    }

    fn find_user_object(username: &str) -> (String, String, String) {
    
        let connection = sqlite::open("users.db").unwrap();
    
        let content = format!("SELECT * FROM users WHERE username = '{}';", username);
        let mut statement = connection.prepare(content).unwrap();
            
        statement.next().unwrap();
    
        let obj_username = statement.read::<String>(0).unwrap();
        let obj_e_mail = statement.read::<String>(1).unwrap();
        let obj_password_hash = statement.read::<String>(2).unwrap();
        
        return (obj_username, obj_password_hash, obj_e_mail);
    }

    fn save_user_object(username: &str, e_mail: &str, password_hash: &str) {
        let connection = sqlite::open("users.db").unwrap();
        
        let content = format!("INSERT INTO users VALUES('{}', '{}', '{}');", username, e_mail, password_hash);
        connection.execute(content).unwrap();
    }
}


pub fn create_table() {
    let connection = sqlite::open("users.db").unwrap();

    connection.execute("CREATE TABLE IF NOT EXISTS users (username TEXT, e_mail TEXT, password_hash TEXT);",).unwrap();
}


pub fn check_username(username: &str) -> bool {
    let connection = sqlite::open("users.db").unwrap();

    let content = format!("SELECT * FROM users WHERE username = '{}';", username);
    let mut statement = connection.prepare(content).unwrap();

    return sqlite::State::Row == statement.next().unwrap();
}


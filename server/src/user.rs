use sha_crypt::{Sha512Params, sha512_simple, sha512_check};
use crate::database;


pub struct User {
    username: String,
    password_hash: String,
    e_mail: String
}

impl User {
    pub fn new(username: &str, e_mail: &str, password: &str) -> Self {
        return Self {
            username: username.to_string(),
            password_hash: Self::create_password_hash(password),
            e_mail: e_mail.to_string()
        };
    }

    pub fn from_database(username: &str) -> Self {
        let user_data = database::find_user_object(username).unwrap();

        return Self {
            username: user_data.0,
            password_hash: user_data.1,
            e_mail: user_data.2
        };
    }

    fn create_password_hash(password: &str) -> String {
        let params: Sha512Params = Sha512Params::new(10_000).unwrap();
        sha512_simple(password, &params).unwrap()
    }

    pub fn password_check(&self, password: &str) -> bool {
        sha512_check(password, &self.password_hash).is_ok()
    }

    pub fn save_user(&self) {
        database::save_user_object(&self.username, &self.e_mail, &self.password_hash);
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }
}


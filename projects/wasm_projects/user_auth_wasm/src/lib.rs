use wasm_bindgen::prelude::*;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, PasswordHasher as _, PasswordVerifier as _};
use getrandom::getrandom;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    pub type SqlJsDatabase;

    #[wasm_bindgen(constructor)]
    fn new() -> SqlJsDatabase;

    #[wasm_bindgen(method)]
    fn exec(this: &SqlJsDatabase, sql: &str);
}

#[wasm_bindgen]
pub struct UserAuth {
    db: SqlJsDatabase,
    users: HashMap<String, String>, // Stores username and hashed password
}

#[wasm_bindgen]
impl UserAuth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UserAuth {
        let db = SqlJsDatabase::new();
        db.exec("CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY, password TEXT);");
        UserAuth {
            db,
            users: HashMap::new(),
        }
    }

    pub fn register(&mut self, username: &str, password: &str) -> bool {
        if self.users.contains_key(username) {
            return false;
        }

        // Generate a salt
        let mut salt = [0u8; 16];
        getrandom(&mut salt).expect("Failed to generate salt");
        let salt_string = SaltString::b64_encode(&salt).unwrap();

        // Hash the password with Argon2
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_string).unwrap().to_string();

        // Store the username and password hash in the database
        self.db.exec(&format!(
            "INSERT INTO users (username, password) VALUES ('{}', '{}');",
            username, password_hash
        ));
        self.users.insert(username.to_string(), password_hash);
        true
    }

    pub fn login(&self, username: &str, password: &str) -> bool {
        if let Some(stored_hash) = self.users.get(username) {
            let parsed_hash = PasswordHash::new(stored_hash).unwrap();
            let argon2 = Argon2::default();
            return argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
        }
        false
    }
}
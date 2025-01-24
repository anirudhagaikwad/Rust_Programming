use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub email: String,
    pub password: String, // Plaintext password for input
}

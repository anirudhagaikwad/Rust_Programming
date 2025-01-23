use serde::Serialize;
use rocket::form::FromForm;

#[derive(Serialize, FromForm)]
pub struct Employee {
    pub id: Option<i32>, // Optional since it's auto-generated
    pub name: String,
    pub position: String,
    pub salary: f64,
}

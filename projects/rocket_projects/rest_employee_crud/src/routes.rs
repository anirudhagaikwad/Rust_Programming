use rocket::{post, get, put, delete, http::Status};
use rocket::serde::{json::Json, Deserialize, Serialize};
use argon2::{password_hash::{SaltString, PasswordHasher}, Argon2, Algorithm, Version, Params};
use crate::db::DB;
use rusqlite::params;

// Define the Employee struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub email: String,
    pub password: String, // Input from the user, not stored directly
}

// Define the API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub message: String,
    pub data: Option<Employee>, // Use Employee directly instead of Json<Employee>
}

// Add a new employee
#[post("/employee", data = "<employee>")]
pub fn create_employee(employee: Json<Employee>) -> Result<Json<ApiResponse>, Status> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    );

    // Generate a secure random salt
    let salt = SaltString::generate(&mut rand::thread_rng());

    // Hash the password
    let password_hash = argon2
        .hash_password(employee.password.as_bytes(), &salt)
        .map_err(|_| Status::InternalServerError)?
        .to_string();

    // Insert into the database
    let conn = DB.get_conn();
    let result = conn.execute(
        "INSERT INTO employees (name, position, email, password_hash) VALUES (?1, ?2, ?3, ?4)",
        params![
            employee.name,
            employee.position,
            employee.email,
            password_hash
        ],
    );

    match result {
        Ok(_) => Ok(Json(ApiResponse {
            message: "Employee added successfully".to_string(),
            data: None,
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Get all employees
#[get("/employee")] // change this from /employees to /employee
pub fn get_employees() -> Result<Json<Vec<Employee>>, Status> {
    let conn = DB.get_conn();
    let mut stmt = conn
        .prepare("SELECT id, name, position, email, '' AS password FROM employees")
        .map_err(|_| Status::InternalServerError)?;

    let employees_iter = stmt
        .query_map([], |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                position: row.get(2)?,
                email: row.get(3)?,
                password: "".to_string(), // Do not expose passwords
            })
        })
        .map_err(|_| Status::InternalServerError)?;

    let employees: Vec<Employee> = employees_iter
        .filter_map(Result::ok)
        .collect();

    Ok(Json(employees))
}

// Update an existing employee
#[put("/employee/<id>", data = "<employee>")]
pub fn update_employee(id: i32, employee: Json<Employee>) -> Result<Json<ApiResponse>, Status> {
    let conn = DB.get_conn();

    // Hash the password if it's provided
    let password_hash = if !employee.password.is_empty() {
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default(),
        );
        let salt = SaltString::generate(&mut rand::thread_rng());
        argon2
            .hash_password(employee.password.as_bytes(), &salt)
            .map_err(|_| Status::InternalServerError)?
            .to_string()
    } else {
        "".to_string() // No change to password
    };

    let result = conn.execute(
        "UPDATE employees SET name = ?1, position = ?2, email = ?3, password_hash = ?4 WHERE id = ?5",
        params![
            employee.name,
            employee.position,
            employee.email,
            password_hash,
            id
        ],
    );

    match result {
        Ok(updated) if updated > 0 => Ok(Json(ApiResponse {
            message: "Employee updated successfully".to_string(),
            data: Some(employee.into_inner()), // Return updated employee data
        })),
        Ok(_) => Err(Status::NotFound), // Employee with the given ID was not found
        Err(_) => Err(Status::InternalServerError),
    }
}

// Delete a specific employee
#[delete("/employee/<id>")]
pub fn delete_employee(id: i32) -> Result<Json<ApiResponse>, Status> {
    let conn = DB.get_conn();

    let result = conn.execute(
        "DELETE FROM employees WHERE id = ?1",
        params![id],
    );

    match result {
        Ok(deleted) if deleted > 0 => Ok(Json(ApiResponse {
            message: "Employee deleted successfully".to_string(),
            data: None,
        })),
        Ok(_) => Err(Status::NotFound), // Employee with the given ID was not found
        Err(_) => Err(Status::InternalServerError),
    }
}

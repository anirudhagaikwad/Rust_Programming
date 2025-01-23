use wasm_bindgen::prelude::*;

// Exposing the `create_employee` function to JS
#[wasm_bindgen]
pub fn create_employee(name: &str, age: i32, department: &str) -> String {
    format!("Employee created: Name: {}, Age: {}, Department: {}", name, age, department)
}

// Function to list employees (for simplicity, we return a static string)
#[wasm_bindgen]
pub fn read_employees() -> String {
    String::from("Employee List: [1. Alice, 2. Bob, 3. Charlie]")
}

// Function to update employee information
#[wasm_bindgen]
pub fn update_employee(id: i32, name: &str, age: i32, department: &str) -> String {
    format!("Employee with ID {} updated: Name: {}, Age: {}, Department: {}", id, name, age, department)
}

// Function to delete an employee
#[wasm_bindgen]
pub fn delete_employee(id: i32) -> String {
    format!("Employee with ID {} deleted.", id)
}

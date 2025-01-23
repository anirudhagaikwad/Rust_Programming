#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::response::{Redirect, content};
use rocket_dyn_templates::{context, Template};
use rusqlite::{params, Connection};
use std::fs;

mod schema;

use schema::Employee;

fn initialize_database() -> Result<(), String> {
    let db_path = "database/employee.db";

    // Create the `database/` directory if it doesn't exist
    if let Err(e) = fs::create_dir_all("database") {
        return Err(format!("Failed to create database directory: {}", e));
    }

    // Check if the database file exists, and create it if not
    if !std::path::Path::new(db_path).exists() {
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;
        conn.execute(
            "CREATE TABLE employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                position TEXT NOT NULL,
                salary REAL NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create employees table: {}", e))?;
        println!("Database and table created successfully!");
    }

    Ok(())
}

#[catch(500)]
fn internal_error(req: &rocket::Request) -> content::RawHtml<String> {
    let message = format!(
        "<h1>500 - Internal Server Error</h1><p>Path: {}</p><p>Error occurred during processing.</p>",
        req.uri()
    );
    content::RawHtml(message)
}

#[get("/")]
fn index() -> Result<Template, String> {
    let conn = Connection::open("database/employee.db").map_err(|e| format!("Failed to open database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT id, name, position, salary FROM employees")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let employee_iter = stmt
        .query_map([], |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                position: row.get(2)?,
                salary: row.get(3)?,
            })
        }).map_err(|e| format!("Failed to map query: {}", e))?;

    let employees: Vec<Employee> = employee_iter.map(|e| e.unwrap_or_else(|_| {
        panic!("Failed to map employee row into Employee struct");
    })).collect();

    Ok(Template::render("index", context! { employees }))
}

#[get("/add")]
fn add_form() -> Template {
    Template::render("addemp", context! {})
}

#[post("/add", data = "<employee_form>")]
fn add_employee(employee_form: Form<Employee>) -> Redirect {
    let employee = employee_form.into_inner();
    let conn = Connection::open("database/employee.db").expect("Failed to open database");

    if let Err(e) = conn.execute(
        "INSERT INTO employees (name, position, salary) VALUES (?1, ?2, ?3)",
        params![employee.name, employee.position, employee.salary],
    ) {
        eprintln!("Failed to insert employee: {}", e);
    }

    Redirect::to("/")
}

#[get("/update/<id>")]
fn update_form(id: i32) -> Result<Template, String> {
    let conn = Connection::open("database/employee.db").map_err(|e| format!("Failed to open database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT id, name, position, salary FROM employees WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let employee = stmt
        .query_row(params![id], |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                position: row.get(2)?,
                salary: row.get(3)?,
            })
        }).map_err(|e| format!("Failed to fetch employee: {}", e))?;

    Ok(Template::render("updateemp", context! { employee }))
}

#[post("/update/<id>", data = "<employee_form>")]
fn update_employee(id: i32, employee_form: Form<Employee>) -> Redirect {
    let employee = employee_form.into_inner();
    let conn = Connection::open("database/employee.db").expect("Failed to open database");

    if let Err(e) = conn.execute(
        "UPDATE employees SET name = ?1, position = ?2, salary = ?3 WHERE id = ?4",
        params![employee.name, employee.position, employee.salary, id],
    ) {
        eprintln!("Failed to update employee: {}", e);
    }

    Redirect::to("/")
}

#[get("/delete/<id>")]
fn delete_employee(id: i32) -> Redirect {
    let conn = Connection::open("database/employee.db").expect("Failed to open database");

    if let Err(e) = conn.execute("DELETE FROM employees WHERE id = ?1", params![id]) {
        eprintln!("Failed to delete employee: {}", e);
    }

    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    if let Err(e) = initialize_database() {
        eprintln!("{}", e);
    }

    rocket::build()
        .mount(
            "/",
            routes![index, add_form, add_employee, update_form, update_employee, delete_employee],
        )
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
        .register("/", catchers![internal_error]) // Register custom error catcher
}

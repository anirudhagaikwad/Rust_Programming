/*
When you run `cargo add rusqlite`, it adds the `rusqlite` crate to your `Cargo.toml` file with the default configuration. However, `rusqlite` has additional optional features that you can enable if you need them.
### Including All Features of `rusqlite`
To include all features, you can use the `--features` flag when adding the crate:

```bash
cargo add rusqlite --features "bundled"
```

### Explanation of Features
1. bundled:
   - This feature bundles SQLite with the `rusqlite` crate, meaning it includes SQLite’s C library as part of your Rust project.
   - It's useful if you don't want to rely on a system-installed version of SQLite.

2. other optional features:
   - Some crates might have additional feature flags (like support for specific database functionalities or performance optimizations).
   - For `rusqlite`, you can look at the available features in its documentation or by running:

```bash
cargo readme |sql-cli.compile=final output provided a breakdown in.documentation support

You can find and enable the features for `rusqlite` by checking its documentation on [crates.io](https://crates.io/crates/rusqlite). Here's how to include all features or specific ones:

### Enabling All Available Features
The `rusqlite` crate does not provide a pre-defined "all features" option, but you can manually specify the features you want to include. Run the following command:

```bash
cargo add rusqlite --features "bundled sqlcipher"
```

### List of Common Features in `rusqlite`
As of the latest version, here are some key features you might enable:
1. `bundled`: Includes the SQLite library statically bundled with your Rust code.
2. `sqlcipher`: Adds support for SQLCipher, enabling encrypted SQLite databases.
3. `serde_json`: Adds support for working with JSON values in SQLite.
4. `time`: Adds support for `chrono` and `time` types.
5. `r2d2`: Adds support for connection pooling using the `r2d2` crate.

### Modifying `Cargo.toml` Manually
If you prefer to add features manually in the `Cargo.toml` file, edit it as follows:

```toml
[dependencies]
rusqlite = { version = "0.29.0", features = ["bundled", "serde_json", "sqlcipher"] }
```

### Check Features in Crate Documentation
1. Visit the `rusqlite` [documentation on crates.io](https://crates.io/crates/rusqlite).
2. Look for the Features section in the README or documentation.

---

### When Should You Use Features?
- Default: The default configuration is enough for simple CRUD operations and SQLite support.
- Additional Features: Use features like `sqlcipher` if you need encryption or `bundled` if you don't want to rely on system-installed SQLite.

*/

use rusqlite::{params, Connection, Result};
use std::io;

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    age: i32,
    department: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("employee.db")?;
    initialize_database(&conn)?;

    loop {
        println!("\nEmployee CRUD Operations:");
        println!("1. Create Employee");
        println!("2. Read Employees");
        println!("3. Update Employee");
        println!("4. Delete Employee");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<i32>().unwrap_or(0);

        match choice {
            1 => create_employee(&conn)?,
            2 => read_employees(&conn)?,
            3 => update_employee(&conn)?,
            4 => delete_employee(&conn)?,
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }

    Ok(())
}

fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS employee (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            department TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn create_employee(conn: &Connection) -> Result<()> {
    let mut name = String::new();
    let mut age = String::new();
    let mut department = String::new();

    println!("Enter employee name: ");
    io::stdin().read_line(&mut name).unwrap();
    println!("Enter employee age: ");
    io::stdin().read_line(&mut age).unwrap();
    println!("Enter employee department: ");
    io::stdin().read_line(&mut department).unwrap();

    let age: i32 = age.trim().parse().unwrap_or(0);

    conn.execute(
        "INSERT INTO employee (name, age, department) VALUES (?1, ?2, ?3)",
        params![name.trim(), age, department.trim()],
    )?;
    println!("Employee added successfully!");

    Ok(())
}

fn read_employees(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age, department FROM employee")?;
    let employee_iter = stmt.query_map([], |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            department: row.get(3)?,
        })
    })?;

    println!("\nEmployee List:");
    for employee in employee_iter {
        let employee = employee.unwrap();
        // Explicitly print out all fields to ensure they're read
        println!(
            "ID: {}, Name: {}, Age: {}, Department: {}",
            employee.id, employee.name, employee.age, employee.department
        );
    }

    Ok(())
}

fn update_employee(conn: &Connection) -> Result<()> {
    let mut id = String::new();
    let mut name = String::new();
    let mut age = String::new();
    let mut department = String::new();

    println!("Enter employee ID to update: ");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap_or(0);

    println!("Enter new name: ");
    io::stdin().read_line(&mut name).unwrap();
    println!("Enter new age: ");
    io::stdin().read_line(&mut age).unwrap();
    println!("Enter new department: ");
    io::stdin().read_line(&mut department).unwrap();

    let age: i32 = age.trim().parse().unwrap_or(0);

    conn.execute(
        "UPDATE employee SET name = ?1, age = ?2, department = ?3 WHERE id = ?4",
        params![name.trim(), age, department.trim(), id],
    )?;

    println!("Employee updated successfully!");

    Ok(())
}

fn delete_employee(conn: &Connection) -> Result<()> {
    let mut id = String::new();

    println!("Enter employee ID to delete: ");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap_or(0);

    conn.execute("DELETE FROM employee WHERE id = ?1", params![id])?;
    println!("Employee deleted successfully!");

    Ok(())
}


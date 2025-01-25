#[macro_use]
extern crate rocket;

mod db;
mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            create_employee,  // Create employee
            get_employees,    // Get all employees
            update_employee,  // Update employee
            delete_employee   // Delete employee
        ]) // Mounting all the routes
}
 
/*

### Tested Routes using POSTMAN
1. Create Employee:  
   - Method: `POST /employee` 
   - URL : http://127.0.0.1:8000/employee 
   - Body (JSON):  
     ```json
     {
       "name": "Anirudha Gaikwad",
       "position": "Software Engineer",
       "email": "ani@example.com",
       "password": "securepassword123"
     }
     ```
   - Response:  
     ```json
     {
       "message": "Employee added successfully",
       "data": null
     }
     ```

2. Get Employees:  
   - Method: `GET /employees`
   - URL : http://127.0.0.1:8000/employee   
   - Response:  
     ```json
     [
       {
         "id": 1,
          
       "name": "Anirudha Gaikwad",
       "position": "Software Engineer",
       "email": "ani@example.com",
       "password": "securepassword123"
     
       }
     ]
     ```

*/

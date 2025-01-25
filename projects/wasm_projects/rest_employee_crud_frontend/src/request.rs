use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request as WebRequest, RequestInit, RequestMode, Response};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Employee {
    pub id: Option<u32>,
    pub name: String,
    pub position: String,
    pub email: String,
    pub password: String,
}

const API_URL: &str = "http://127.0.0.1:8000/employee";

/// Fetch all employees
pub async fn get_all_employees() -> Result<Vec<Employee>, String> {
    let mut opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = WebRequest::new_with_str_and_init(API_URL, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap()
        .dyn_into::<Response>()
        .unwrap();

    let json = JsFuture::from(response.json().unwrap())
        .await
        .unwrap();
    let employees: Vec<Employee> = json.into_serde().unwrap();
    Ok(employees)
}

/// Add a new employee
pub async fn add_employee(employee: Employee) -> Result<(), String> {
    let mut opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(Some(&JsValue::from_str(
        &serde_json::to_string(&employee).unwrap(),
    )));

    let request = WebRequest::new_with_str_and_init(API_URL, &opts).unwrap();
    let window = web_sys::window().unwrap();
    JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    Ok(())
}

/// Delete an employee
pub async fn delete_employee(id: u32) -> Result<(), String> {
    let url = format!("{}/{}", API_URL, id);
    let mut opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);

    let request = WebRequest::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    Ok(())
}

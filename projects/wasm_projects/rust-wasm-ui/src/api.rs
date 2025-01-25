use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use anyhow::{Result, anyhow};


const API_BASE_URL: &str = "http://127.0.0.1:8000/employee";

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub email: String,
    pub password: Option<String>,
}
#[derive(Debug)]
pub enum ApiError {
    RequestFailed(String),
    ParseFailed(String),
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            ApiError::ParseFailed(msg) => write!(f, "Parse failed: {}", msg),
        }
    }
}

// Function to handle API calls and return Result
pub async fn fetch_api<T: for<'de> Deserialize<'de>, B: Serialize>(
    url: &str,
    method: &str,
    body: Option<B>,
) -> Result<T,ApiError> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);

    if let Some(body) = body {
        let json_body = serde_json::to_string(&body).map_err(|err| {
            ApiError::RequestFailed(format!("Serialization error: {}", err))
        })?;
        opts.body(Some(&JsValue::from_str(&json_body)));
        let headers = web_sys::Headers::new().unwrap();
        headers.set("Content-Type", "application/json").unwrap();
         opts.headers(&headers);
    }

    let request = Request::new_with_str_and_init(url, &opts)
       .map_err(|err| ApiError::RequestFailed(format!("Failed to create request: {:?}", err)))?;

    let window = web_sys::window().ok_or( ApiError::RequestFailed("Failed to get window".to_string()))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|err| ApiError::RequestFailed(format!("Request failed : {:?}", err)))?;

    let resp: Response = resp_value.dyn_into().map_err(|err| ApiError::RequestFailed(format!("Failed to cast response: {:?}", err)))?;

    if !resp.ok() {
      let err_text = JsFuture::from(resp.text().map_err(|e| ApiError::RequestFailed(format!("Failed to get response text: {:?}",e)))?).await.map_err(|e| ApiError::RequestFailed(format!("Failed to get response text: {:?}",e)))?;
      let error_message = format!("API request failed with status {}: {}", resp.status(), err_text.as_string().unwrap_or_default());
        return Err(ApiError::RequestFailed(error_message));
    }


    let json = JsFuture::from(resp.json().map_err(|e| ApiError::ParseFailed(format!("Failed to get json: {:?}",e)))?).await
        .map_err(|err|  ApiError::ParseFailed(format!("Failed to convert json to JsValue: {:?}", err)))?;


    let result: T = serde_wasm_bindgen::from_value(json).map_err(|err| ApiError::ParseFailed(format!("Failed to parse response body: {:?}", err)))?;

    Ok(result)
}

pub async fn get_employees() -> Result<Vec<Employee>,ApiError> {
    fetch_api::<Vec<Employee>, ()>(API_BASE_URL, "GET", None).await
}

pub async fn create_employee(employee: &Employee) -> Result<Employee,ApiError> {
    fetch_api::<Employee, _>(API_BASE_URL, "POST", Some(employee)).await
}

pub async fn update_employee(employee_id: &str, employee: &Employee) -> Result<Employee,ApiError> {
    let url = format!("{}/{}", API_BASE_URL, employee_id);
    fetch_api::<Employee, _>(&url, "PUT", Some(employee)).await
}

pub async fn delete_employee(employee_id: &str) -> Result<(),ApiError> {
     let url = format!("{}/{}", API_BASE_URL, employee_id);
     fetch_api::<(), ()>(&url, "DELETE", None).await
}
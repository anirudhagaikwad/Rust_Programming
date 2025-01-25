use yew::prelude::*;
use crate::api::{Employee, create_employee, update_employee,ApiError};
use yew_router::prelude::*;


#[derive(Properties, PartialEq)]
pub struct EmployeeFormProps {
    #[prop_or_default]
    pub employee_id: String,
}

#[function_component(EmployeeForm)]
pub fn employee_form(props: &EmployeeFormProps) -> Html {
    let navigator = use_navigator().unwrap();
    let employee_id = props.employee_id.clone();

    let employee = use_state(|| Employee {
        id: None,
        name: String::new(),
        position: String::new(),
        email: String::new(),
        password: Some(String::new()),
    });

     let error_message = use_state(|| None);

    {
        let employee = employee.clone();
        let error_message = error_message.clone();
        use_effect_with_deps(
            move |employee_id|{
                let employee = employee.clone();
                 let error_message = error_message.clone();
                if !employee_id.is_empty() {
                    wasm_bindgen_futures::spawn_local(async move {
                         match crate::api::get_employees().await {
                            Ok(employees) => {
                               if let Some(emp) = employees.iter().find(|e| e.id == employee_id.parse().ok()) {
                                     employee.set(emp.clone());
                               }else {
                                error_message.set(Some("Employee not found".to_string()));
                               }
                            },
                            Err(e) => {
                                error_message.set(Some(format!("Error fetching employee for edit: {}", e)));
                            }

                        }

                    });
                }
            },
           employee_id
        )
    }


    let onchange_name = {
        let employee = employee.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlInputElement>().value();
            let mut new_employee = (*employee).clone();
            new_employee.name = value;
            employee.set(new_employee);
        })
    };

    let onchange_position = {
        let employee = employee.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlInputElement>().value();
             let mut new_employee = (*employee).clone();
             new_employee.position = value;
            employee.set(new_employee);
        })
    };

    let onchange_email = {
        let employee = employee.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlInputElement>().value();
             let mut new_employee = (*employee).clone();
             new_employee.email = value;
            employee.set(new_employee);
        })
    };

     let onchange_password = {
        let employee = employee.clone();
         Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlInputElement>().value();
            let mut new_employee = (*employee).clone();
            new_employee.password = Some(value);
            employee.set(new_employee);
        })
    };
  
    let onsubmit = {
         let error_message = error_message.clone();
        let employee = employee.clone();
        let navigator = navigator.clone();
        let employee_id = employee_id.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
             let employee = (*employee).clone();
             let navigator = navigator.clone();
            let error_message = error_message.clone();
            let employee_id = employee_id.clone();
            wasm_bindgen_futures::spawn_local(async move {

               let result = if employee_id.is_empty() {
                     create_employee(&employee).await
                }else {
                    update_employee(&employee_id,&employee).await
                };


                match result {
                    Ok(_) => {
                        navigator.push(&crate::app::Route::Home);
                    }
                    Err(e) => {
                        let message = match e{
                            ApiError::RequestFailed(msg) => msg,
                            ApiError::ParseFailed(msg)=> msg,
                        };
                        error_message.set(Some(format!("Error in form: {}", message)));
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{if employee_id.is_empty() { "Add Employee"} else { "Edit Employee" }}</h1>
             {if let Some(msg) = &*error_message {
                html! { <p class="error">{msg}</p> }
             } else {
                 html! {}
             }}
            <form onsubmit={onsubmit}>
                <div class="form-group">
                    <label for="name">{"Name"}</label>
                    <input type="text" id="name" value={employee.name.clone()} onchange={onchange_name} required=true />
                </div>
                <div class="form-group">
                    <label for="position">{"Position"}</label>
                    <input type="text" id="position" value={employee.position.clone()} onchange={onchange_position} required=true />
                </div>
                <div class="form-group">
                    <label for="email">{"Email"}</label>
                    <input type="email" id="email" value={employee.email.clone()}  onchange={onchange_email}  required=true/>
                </div>
                 { if employee_id.is_empty() {
                    html! {
                         <div class="form-group">
                         <label for="password">{"Password"}</label>
                         <input type="password" id="password" onchange={onchange_password} required=true/>
                         </div>
                     }
                 } else {
                   html!{}
                 }
                }
               
                <button type="submit" class="primary-button">
                    {if employee_id.is_empty() { "Add Employee"} else { "Update Employee" }}
                 </button>
            </form>
        </div>
    }
}
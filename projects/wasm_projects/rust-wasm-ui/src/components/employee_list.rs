use yew::prelude::*;
use crate::api::{Employee, get_employees, delete_employee, ApiError};
use crate::components::employee_card::EmployeeCard;
use yew_router::prelude::*;

#[function_component(EmployeeList)]
pub fn employee_list() -> Html {
    let employees = use_state(|| Vec::<Employee>::new());
    let error_message = use_state(|| None);
      let navigator = use_navigator().unwrap();
   {
        let employees = employees.clone();
        let error_message = error_message.clone();
        use_effect_with_deps(
             move |_| {
                let employees = employees.clone();
                let error_message = error_message.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match get_employees().await {
                        Ok(emps) => {
                            employees.set(emps);
                         },
                        Err(e) => {
                            error_message.set(Some(format!("Error fetching employees: {}", e)));
                        }
                    }
                });
            },
            (),
        );
    }


    let on_delete = {
        let employees = employees.clone();
        let error_message = error_message.clone();
        Callback::from(move |employee_id: String| {
             let employees = employees.clone();
            let error_message = error_message.clone();
             wasm_bindgen_futures::spawn_local(async move {
               match  delete_employee(&employee_id).await {
                   Ok(_) => {
                        match get_employees().await {
                            Ok(emps) => {
                                employees.set(emps);
                            },
                            Err(e) => {
                                error_message.set(Some(format!("Error fetching employees after delete: {}", e)));
                            }
                        }
                   },
                   Err(e) => {
                        error_message.set(Some(format!("Error deleting employee: {}", e)));
                   }
               }
           });
        })
    };

    let onclick_add = {
       let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::app::Route::AddEmployee);
        })
   };


    html! {
        <div>
            <h1>{"Employee List"}</h1>
           {if let Some(msg) = &*error_message {
                html! { <p class="error">{msg}</p> }
            } else {
                 html! {}
            }}
            <ul>
                { for employees.iter().map(|employee| {
                   let on_delete = on_delete.clone();
                   html! {
                        <EmployeeCard  employee={employee.clone()}  on_delete={on_delete} />
                   }
                })}
            </ul>
            <button onclick={onclick_add} class="primary-button">{"Add New Employee"}</button>
        </div>
    }
}
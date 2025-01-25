use crate::request::{get_all_employees, Employee};
use yew::prelude::*;

#[function_component(EmployeeList)]
pub fn employee_list() -> Html {
    let employees = use_state(Vec::<Employee>::new);

    {
        let employees = employees.clone();
        use_effect_with(move || {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(data) = get_all_employees().await {
                    employees.set(data);
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <h2>{ "Employee List" }</h2>
            <ul>
                { for employees.iter().map(|employee| html! {
                    <li key={employee.id.unwrap_or_default()}>
                        { format!("{} - {}", employee.name, employee.position) }
                    </li>
                }) }
            </ul>
        </div>
    }
}

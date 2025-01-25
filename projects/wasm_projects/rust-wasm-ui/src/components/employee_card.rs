use yew::prelude::*;
use crate::api::Employee;

#[derive(Properties, PartialEq)]
pub struct EmployeeCardProps {
    pub employee: Employee,
    pub on_delete: Callback<String>,
}

#[function_component(EmployeeCard)]
pub fn employee_card(props: &EmployeeCardProps) -> Html {
    let employee = &props.employee;
    let on_delete = props.on_delete.clone();
    let id = employee.id.map(|id| id.to_string()).unwrap_or_default();

    let delete = {
        let id = id.clone();
        Callback::from(move |_| {
            on_delete.emit(id.clone());
        })
    };

    html! {
        <li class="employee-card">
            <div class="employee-card-details">
                <p>
                    <strong>{"Name:"}</strong>
                    {" "}{&employee.name}
                </p>
                <p>
                    <strong>{"Position:"}</strong>
                    {" "}{&employee.position}
                </p>
                <p>
                    <strong>{"Email:"}</strong>
                    {" "}{&employee.email}
                </p>
            </div>
            <button onclick={delete}>{"Delete"}</button>
        </li>
    }
}

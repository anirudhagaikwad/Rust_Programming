use crate::request::{add_employee, Employee};
use yew::prelude::*;

#[function_component(EmployeeForm)]
pub fn employee_form() -> Html {
    let name = use_state(|| "".to_string());
    let position = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let on_submit = {
        let name = name.clone();
        let position = position.clone();
        let email = email.clone();
        let password = password.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let employee = Employee {
                id: None,
                name: (*name).clone(),
                position: (*position).clone(),
                email: (*email).clone(),
                password: (*password).clone(),
            };
            wasm_bindgen_futures::spawn_local(async move {
                add_employee(employee).await.unwrap();
            });
        })
    };

    html! {
        <form {on_submit}>
            <input
                type="text"
                placeholder="Name"
                value={(*name).clone()}
                oninput={Callback::from(move |e: InputEvent| name.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
            />
            <input
                type="text"
                placeholder="Position"
                value={(*position).clone()}
                oninput={Callback::from(move |e: InputEvent| position.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
            />
            <input
                type="email"
                placeholder="Email"
                value={(*email).clone()}
                oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
            />
            <input
                type="password"
                placeholder="Password"
                value={(*password).clone()}
                oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
            />
            <button type="submit">{ "Add Employee" }</button>
        </form>
    }
}

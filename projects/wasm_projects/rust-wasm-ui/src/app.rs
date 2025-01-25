use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::employee_list::EmployeeList;
use crate::components::employee_form::EmployeeForm;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    AddEmployee,
    #[at("/edit/:id")]
    EditEmployee { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <EmployeeList /> },
        Route::AddEmployee => html! { <EmployeeForm /> },
        Route::EditEmployee { id } => html! { <EmployeeForm employee_id={Some(id)} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="nav-bar">
                <nav class="nav-links">
                    <Link<Route> to={Route::Home}>{ "Employee List" }</Link<Route>>
                    <Link<Route> to={Route::AddEmployee}>{ "Add Employee" }</Link<Route>>
                </nav>
            </div>
            <div class="container">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

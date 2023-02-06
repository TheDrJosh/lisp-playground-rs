use about::About;
use common::Header;
use login::Login;
use signup::Signup;
use yew::prelude::*;
use yew_router::prelude::*;
mod filler;
mod home;
mod about;
mod login;
mod common;
mod user_info;
mod signup;

//look at https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/node_refs



#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/explore")]
    Explore,
    #[at("/about")]
    About,
    #[at("/editor")]
    Editor,
    #[at("/login")]
    LogIn,
    #[at("/signup")]
    SignUp,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <home::Home/>
        },
        Route::Explore => html! {
            <Header/>
        },
        Route::About => html! {
            <About/>
        },
        Route::Editor => todo!(),
        Route::LogIn => html! {<Login/>},
        Route::SignUp => html! {<Signup/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

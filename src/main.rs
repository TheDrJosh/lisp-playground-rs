mod home;
use yew::prelude::*;
use yew_router::prelude::*;
mod filler;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/Explore")]
    Explore,
    #[at("/About")]
    About,
    #[at("/Editor")]
    Editor,
    #[not_found]
    #[at("/404")]
    NotFound,
}
use web_sys::HtmlInputElement;


#[function_component]
fn ProjectListing() -> Html {
    html! {
        <div class="project_listing">
            <h2 class="project_title">{"Title"}</h2>
            <p class="desc">{ filler::TEXT }</p>
        </div>
    }
}

#[function_component]
fn Acount() -> Html {
    if false {
        html! {
            <div class="acount">
                <img src="https://yew.rs/img/logo.svg"/>
                <h3 class="acount_name">{ "Acount Name" }</h3>
            </div>
        }
    } else {
        html! {
            <button>{"Login"}</button>
        }
    } 
    
}


#[function_component]
fn Home() -> Html {

    let onclick_home = Callback::from(move |_| {
        web_sys::console::log_1(&"Home".into());
    });
    let onclick_explore = Callback::from(move |_| {
        web_sys::console::log_1(&"Explore".into());
    });
    let onclick_about = Callback::from(move |_| {
        web_sys::console::log_1(&"About".into());
    });
    let onclick_open_project = Callback::from(move |_| {
        web_sys::console::log_1(&"Open Project".into());
    });
    let onclick_new_project = Callback::from(move |_| {
        web_sys::console::log_1(&"New Project".into());
    });

    html! {
        <main>
            <div class="top_bar">
                <h1 class="title">{ "Lisp Playground" }</h1>
                <div class="spacer"></div>
                <Acount/>
            </div>
            <div class="tool_bar">
                <button onclick={onclick_home}>{ "Home" }</button>
                <button onclick={onclick_explore}>{ "Explore" }</button>
                <button onclick={onclick_about}>{ "About" }</button>
                <div class="spacer"></div>
                <button onclick={onclick_open_project}>{ "Open Project" }</button>
                <button onclick={onclick_new_project}>{ "New Project" }</button>
            </div>
            <p class="description">
                {"Lisp Playground is a site writen in rust that allows you to play with lisp"}
            </p>
            <h2>{ "Featured" }</h2>
            <div class="featured">
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
                <ProjectListing/>
            </div>
        </main>
    }
    htm
    
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Explore => todo!(),
        Route::About => todo!(),
        Route::Editor => todo!(),
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
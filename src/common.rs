use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{filler, Route, user_info::UserInfo};

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <TopBar/>
            <NavBar/>
        </header>
    }
}

#[function_component]
pub fn TopBar() -> Html {
    let navigator = use_navigator().unwrap();

    let title_click = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <top>
            <h1 class="title" onclick={title_click}>{ "Lisp Playground" }</h1>
            <div class="spacer"></div>
            <Acount/>
        </top>
    }
}

#[function_component]
pub fn NavBar() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };
    let go_explore_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Explore));
        html! {
            <button {onclick}>{"Explore"}</button>
        }
    };
    let go_about_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::About));
        html! {
            <button {onclick}>{"About"}</button>
        }
    };
    let onclick_open_project = Callback::from(move |_| {
        log!("Open Project");
    });
    let onclick_new_project = Callback::from(move |_| {
        log!("New Project");
    });

    let user_data = use_state(|| Option::<UserInfo>::None);



    html! {
        <nav_bar>
            {go_home_button}
            {go_explore_button}
            {go_about_button}
            <div class="spacer"/>
            if (*user_data).is_none() {
                <button onclick={onclick_open_project}>{ "Open Project" }</button>
            } else {
                <div class="spacer1"/>
            }
            <button onclick={onclick_new_project}>{ "New Project" }</button>
        </nav_bar>
    }
}

#[function_component]
pub fn ProjectListing() -> Html {
    html! {
        <div class="project_listing">
            <h2 class="project_title">{"Title"}</h2>
            <p class="desc">{ filler::TEXT }</p>
        </div>
    }
}




#[function_component]
fn Acount() -> Html {
    let navigator = use_navigator().unwrap();

    let user_info = use_state(|| Option::<UserInfo>::None);

    if user_info.is_some() {
        html! {
            <div class="acount">
                <img src="https://yew.rs/img/logo.svg"/>
                <h3 class="acount_name">{ "Acount Name" }</h3>
            </div>
        }
    } else {
        let onclick_login;
        {
            let navigator = navigator.clone();
            onclick_login = Callback::from(move |_| navigator.push(&Route::LogIn));
        }
        let onclick_signup = Callback::from(move |_| navigator.push(&Route::SignUp));

        html! {
            <>
                <button class="login_button" onclick={onclick_login}>{"Log in"}</button>
                <p style="margin: auto;">{"or"}</p>
                <button class="login_button" onclick={onclick_signup}>{"Sign up"}</button>
            </>
        }
    }
}
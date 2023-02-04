mod home;
use yew::prelude::*;
use web_sys::HtmlInputElement;


#[function_component]
fn ProjectListing() -> Html {
    html! {
        <div class="project_listing">
            <h2>{"Title"}</h2>
            <p>{"desc"}</p>
        </div>
    }
}


#[function_component]
fn Acount() -> Html {
    html! {
        <div class="acount">
            <img src="https://yew.rs/img/logo.svg"/>
            <h3>{ "Acount Name" }</h3>
        </div>
        
    }
}

#[function_component]
fn App() -> Html {
    let test = NodeRef::default();
    let htm = html! {
        <>
            <input type="checkbox" name="logged_in" value="logged_in" ref={&test} />
            <label for="logged_in"> {"Logged In?"}</label><br/>
            <home::Home logged_in={true} />
        </>
    };
    if let Some(input) = test.cast::<HtmlInputElement>() {
         input.value();
    }
    htm
}

fn main() {
    yew::Renderer::<App>::new().render();
}
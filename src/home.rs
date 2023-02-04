use yew::prelude::*;




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

#[derive(PartialEq, Properties)]
pub struct HomeProps {
    pub logged_in: bool,
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    html! {
        <main>
            <div class="top_bar">
                <h1 class="title">{"Lisp Playground"}</h1>
                <div class="spacer"/>
                <Acount/>
            </div>
            <div class="tool_bar">
                <button>{"Home"}</button>
                <button>{"Explore"}</button>
                <button>{"About"}</button>
                <div class="spacer"/>
                if props.logged_in {
                    <button>{"Open Projects"}</button>
                } else {
                    <div class="spacer1"/>
                }
                <button>{"New Project"}</button>
            </div>
            <p class="description">
                {"Lisp Playground is a site writen in rust that allows you to play with lisp"}
            </p>
            <div class="featured">
                <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
                // <ProjectListing/>
            </div>
        </main>    
    }
}
use yew::prelude::*;

use crate::common::{ProjectListing, Header};




#[function_component]
pub fn Home() -> Html {

    html! {
        <home>
            <Header/>
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
        </home>
    }
}

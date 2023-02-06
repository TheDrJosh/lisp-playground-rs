use yew::prelude::*;

use crate::{filler, common::Header};


#[function_component]
pub fn About() -> Html {
    html! {
        <main>
            <Header/>
            <h2 style="margin-left:1em;">{"About"}</h2>
            <p style="font-size: 0.75em; margin: 2rem;">{filler::TEXT}</p>
        </main>
    }
}
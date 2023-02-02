use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <div class="top">
                <h1 class="title">{"Lisp Playground"}</h1>
                <div class="acount">{"acount"}</div>
            </div>
            <p class="disc">
                {"A web rust program that is a playground for the lisp programing language."}
            </p>
            <div class="featured_folder">
                <div class="featured">
                    <h2>{"Test"}</h2>
                    <p>{"a short description."}</p>
                </div>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
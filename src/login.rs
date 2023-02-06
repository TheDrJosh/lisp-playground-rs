use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Login() -> Html {
    let navigator = use_navigator().unwrap();

    let title_click = Callback::from(move |_| navigator.push(&Route::Home));
    let username_text = NodeRef::new();

    html! {
            <signup>
                <header>
                    <h1 onclick={title_click}>{"Lisp Playground"}</h1>
                </header>
                <div class="centered_box">
                    <h3>{"Log In"}</h3>
                    <div>
                        <div>
                            <label for="username">{"Email or Username:"}</label>
                            <input type="text" id="username" name="username"/>
                        </div>

                        <div>
                            <label for="pass">{"Password (8 characters minimum):"}</label>
                            <input type="password" id="pass" name="password" minlength="8" required=true/>
                        </div>

                        <button>{"Log in"}</button>
                    </div>

                    <h3 style="font-size: 1em;">{"or"}</h3>
                    <button>{"Login with Github"}</button>
                    <button>{"Login with Google"}</button>
                    <p style="font-size:0.75em;">{"Don't have an account? "}<Link<Route> to={Route::SignUp}>{"Sign Up"}</Link<Route>></p>
                    <p style="font-size:0.75em;">{"Forgot your password? Reset your password"}</p>


                </div>
            </signup>
        }
}

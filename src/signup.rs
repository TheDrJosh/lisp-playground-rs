use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;


#[function_component]
pub fn Signup() -> Html {
    let navigator = use_navigator().unwrap();

    let title_click = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
            <login>
                <header>
                    <h1 onclick={title_click}>{"Lisp Playground"}</h1>
                </header>
                <div class="centered_box">
                    <h3>{"Sign Up"}</h3>
                    <div>
                        <div>
                            <label for="username">{"User Name:"}</label>
                            <input type="text" id="username" name="username"/>
                        </div>
                        <div>
                            <label for="email">{"Email:"}</label>
                            <input type="text" id="email" name="email"/>
                        </div>
                        <div>
                            <label for="password">{"Password (8 characters minimum):"}</label>
                            <input type="password" id="password" name="password" minlength="8" required=true/>
                        </div>
                        <div>
                            <label for="cpassword">{"Confirm Password:"}</label>
                            <input type="password" id="cpassword" name="cpassword" minlength="8" required=true/>
                        </div>
                        <button>{"Log in"}</button>
                    </div>
                    <h3 style="font-size: 1em;">{"or"}</h3>
                    <button>{"Login with Github"}</button>
                    <button>{"Login with Google"}</button>
                    <p style="font-size:0.75em;">{"Already have an account? "}<Link<Route> to={Route::SignUp}>{"Log In"}</Link<Route>></p>


                </div>
            </login>
        }
}
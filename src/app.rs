use yew::prelude::*;

use gloo_net::http::*;
use web_sys::HtmlInputElement;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Possible States, excluding None
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum State {
    Good { message: String },
    NotGood { error: String },
    Processing { message: String },
}

// Use as name/value pairs for JSON message body
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PostMessage {
    json: String,
}

// Your app
#[function_component(App)]
pub fn app() -> Html {

    // State object
    let check_state = use_state_eq::<Option<State>, _>(|| None);
    let check_state_outer = check_state.clone();

    // Button state
    let check_button = use_state(|| false);
    let checker = use_state(|| true);

    // Text field
    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();

    // What happens when your button is clicked
    let onclick = Callback::from(move |_| {
        let check_state_clone = check_state.clone();
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let field_input = input.value();

        // Simple cooldown implementation, could be built out with timer/callback/custom Yew hooks
        if check_button.eq(&checker) { 
            check_state_clone.set(Some(State::Processing { message: "Cooldown triggered to prevent spam. Please refresh your browser and try again.".to_string()}));
            return 
        } else {
            check_button.set(true);
        };

        // Used to display a "processing request" message while your app waits for a response from your API
        check_state_clone.set(Some(State::Processing {message: "Make sure you put in your own api url!".to_string()}));

        // Your message body
        let post = PostMessage { 
            json: field_input 
        };

        // Creating your request
        // You may want to add some sort of guard here on serialization errors
        if JsValue::from_serde(&post).is_ok() {
            let opts = Request::new("https://your_http_api_to_post_to.com")
                .json(&post).unwrap()
                .header("Content-Security-Policy", "Your CSP")
                .method(Method::POST);

            // Sending your request
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(x) = opts.send().await {
                    let result = x.status_text();
                    if result == *"OK".to_string() {
                        check_state_clone.set(Some(State::Good {message: "Your success message".to_string()}));
                    } else {
                        check_state_clone.set(Some(State::NotGood {error: "Your error message".to_string()}));
                    }
                }
            })
        };
    });

    // HTML 
    html! {
        <>
            <h1>{ "Your example website" }</h1>
            <h2>{ "Your example header" }</h2>
            <div class ="container">
                <input ref={input_ref_outer.clone()} type="text" id="placeholder" placeholder="placeholder..." autocomplete="off" />
                <button class ="button1" onclick={onclick}>{"Button"}</button>
            </div>
            <div class ="response_container">
            <ViewResponse prop={(*check_state_outer).clone()} />
            </div>
            <body>
            </body>
            <div class ="footer">
                <p>{ "Made by:     "}
                <a href="https://github.com/LeTurt333">{ "LeTurt"}</a>
                </p>
            </div>


        </>
    }
}

// Used for displaying UI messages
#[derive(Properties, PartialEq)]
pub struct ViewProperties {
    prop: Option<State>,
}

// The component to be displayed
#[function_component(ViewResponse)]
fn view_response(props: &ViewProperties) -> Html {
    let response = match &props.prop {
        None => return html! {},
        Some(State::Processing { message }) => message.to_string(),
        Some(State::Good { message }) => format!("Success! {}", message.clone()),
        Some(State::NotGood { error }) => {
            format!("Error: {}", error)
        }
    };

    html! {
        <div>{ response }</div>
    }
}

use std::{borrow::Borrow, collections::HashMap, ops::Deref};

use web_sys::{js_sys::{Error, JSON}, window, HtmlInputElement};
use yew::{
    prelude::*,
    function_component, html, Html, Properties
};
use yew_router::prelude::*;

use reqwest::{header::COOKIE, Client};
use wasm_bindgen_futures::spawn_local;
use gloo::{console::log as clog, events::EventListener, net::http::Request};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Properties, PartialEq, Clone)]
struct SessionToken {
    value: String
}

#[derive(Routable, Debug, Clone, Copy, PartialEq)]
pub enum Route {
    #[at("/page/home")]
    Home,
    #[at("/page/editor/login")]
    EditorLogin,
    #[at("/page/editor/dashboard")]
    EditorDashboard,
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::EditorLogin => html! {
            <EditorLogin />
        },
        Route::EditorDashboard => html! { <h1>{ "EditorDashboard" }</h1> },
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginForm {
    username: String,
    password: String,
}
#[function_component(EditorLogin)]
fn editor_login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let session_token = use_state(|| SessionToken {
        value: String::new()
    });


    let onsubmit = {
        let username = username.clone().to_string();
        let password = password.clone().to_string();
        let session_token = session_token.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let mut params = HashMap::new();
            params.insert("username", username.to_string());
            params.insert("password", password.to_string());
            let session_token = session_token.clone();
            
            spawn_local(async move {
                let client = Client::new();
                let res = client.post("http://127.0.0.2:8081/editor/login")
                    .header("Origin", "http://127.0.0.1:8090")
                    .form(&params)
                    .send()
                    .await;

                match res {
                    Ok(response) => {
                        if response.status().is_success() {
                            clog!("Success to submit form: {:?}");
                            let session_response = response.text().await.expect("Failed to read response body");
                            let mut session_token_cl = session_token.deref().clone();
                            session_token_cl.value = session_response;
                            session_token.set(session_token_cl);
                        } else {
                            clog!("Failed to submit form: {:?}");
                        }
                    }
                    Err(err) => {
                        clog!("Request failed: {:?}");
                    }
                }
            });

        })
    };

    let onusernamechange = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let onpasswordchange = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    let dashboard_request = {
        let session_token = session_token.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let session_token = session_token.clone();
            spawn_local(async move {
                clog!(format!("session_token.value {:?}", session_token.value));
                let client = Client::new();
                let res = client.get("http://127.0.0.2:8081/editor/dashboard")
                    .header("Origin", "http://127.0.0.1:8090")
                    .send()
                    .await;

                match res {
                    Ok(response) => {
                        if response.status().is_success() {
                            clog!("dashboard-request: success ok()");
                        } else {
                            clog!("dashboard-request: not found");
                        }
                    }
                    Err(err) => {
                        clog!("dashboard-request: failed request");
                    }
                }
            });
        })
    };

    html! {
        <>
        <form onsubmit={onsubmit}>
            <div>
                <label for="username">{"Username:"}</label>
                <input
                    type="text"
                    id="username"
                    oninput={onusernamechange}
                    value={(*username).clone()}
                />
            </div>
            <div>
                <label for="password">{"Password:"}</label>
                <input
                    type="password"
                    id="password"
                    oninput={onpasswordchange}
                    value={(*password).clone()}
                />
            </div>
            <button type="submit">{"Login"}</button>
        </form>
        <button onclick={dashboard_request}>{"request dashboard"}</button>
        </>
    }
}

#[function_component(NavigateButton)]
pub fn navigate_button() -> Html {
    let on_click = Callback::from(move |_| {
        if let Some(window) = window() {
            window.location().set_href("/new-page").expect("Failed to set href");
        }
    });

    html! {
        <button onclick={on_click.clone()}>{"Navigate to New Page"}</button>
    }
}
#[function_component(App)]
pub fn app() -> Html {

    html! {
        <BrowserRouter>
            <div>
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><NavigateButton /></li>
                    </ul>
                </nav>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
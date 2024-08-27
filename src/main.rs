use std::{borrow::Borrow, collections::HashMap, ops::Deref, rc::Rc};

use http::{header, HeaderMap, StatusCode};
use wasm_bindgen::JsValue;
use web_sys::{js_sys::{Error, JSON}, window, HtmlInputElement};
use yew::{
    prelude::*,
    function_component, html, Html, Properties
};
use yew_router::prelude::*;
use yew::suspense::SuspensionResult;
use yew::suspense::Suspension;
use reqwest::{header::COOKIE, Client, Url};
use wasm_bindgen_futures::spawn_local;
use gloo::{console::log as clog, events::EventListener, net::http::Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Properties, PartialEq, Clone)]
struct SessionToken {
    value: String
}

#[derive(Routable, Debug, Clone, Copy, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editor/login")]
    EditorLogin,
    #[at("/editor/dashboard")]
    EditorDashboard
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
        },
        Route::EditorLogin => html! {
            <EditorLogin />
        },
        Route::EditorDashboard => html! {
            <EditorDashboard />
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginForm {
    username: String,
    password: String,
}
async fn login(params_json: &Value) -> Option<User> {

    let url_str = "http://127.0.0.2:8081/editor/login";
    let req = reqwasm::http::Request::post(url_str)
    .header("content-type", "applcation/json")
    .body(params_json.to_string())
    .mode(reqwasm::http::RequestMode::NoCors)
    .send()
    .await;
    clog!(format!("req: {:?}", req));

    match req {
        Ok(response) => {
            if response.status_text() == "OK" {
                clog!("Success to submit form");
                //let session_response = response.text().await.expect("Failed to read response body");
                //let mut session_token_cl = session_token.deref().clone();
                //session_token_cl.value = session_response;
                //session_token.set(session_token_cl);
                //
                //has_user_state.set(HasUser{ user: Some(User {
                //    id: 1,
                //    username: "str".to_owned(),
                //    token: "tok".to_owned()
                //})});
                let user = response.json::<User>().await.expect("err");
                return Some(user);
            } else {
                clog!("Non-ok status");
                return None;
            }
        }
        Err(err) => {
            clog!("response err");
            return None;
        }
    }

}

#[function_component(EditorDashboard)]
fn editor_dashboard() -> Html {
    html! {
        <div>{"hello from dashboard"}</div>
    }
}
//LINK - EditorLogin
#[function_component(EditorLogin)]
fn editor_login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let session_token = use_state(|| SessionToken {
        value: String::new()
    });

    let user_state_context = use_context::<UserStateContext>().unwrap();

    let onsubmit = {
        let has_user_ctx = user_state_context.clone();
        let username = username.clone().to_string();
        let password = password.clone().to_string();
        let session_token = session_token.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let has_user_ctx = has_user_ctx.clone();
            let username = username.clone().to_string();
            let password = password.clone().to_string();
            let session_token = session_token.clone();
            let params_json = json!({
                "username": username.to_string(),
                "password": password.to_string()
            });
            clog!(format!("params_json {:?}", &params_json));
            
            spawn_local(async move {
                let usere = login(&params_json).await;
                clog!(format!("user {:?}", &usere));
                has_user_ctx.dispatch(usere);
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
    //let oncheckuser = {
    //    let has_user_state = has_user_state.clone();
    //    Callback::from(move |e: MouseEvent| {
    //        clog!(format!("has_useer {:?}", has_user_state));
    //        e.prevent_default();
    //    })
    //};
    //let dashboard_request = {
    //    let session_token = session_token.clone();
    //    Callback::from(move |e: MouseEvent| {
    //        e.prevent_default();
    //        let session_token = session_token.clone();
    //        spawn_local(async move {
    //            clog!(format!("session_token.value {:?}", session_token.value));
    //            let client = Client::new();
    //            let res = client.get("http://127.0.0.3:8081/editor/dashboard")
    //                .header("Origin", "http://127.0.0.2:8081")
    //                .send()
    //                .await;
//
    //            match res {
    //                Ok(response) => {
    //                    if response.status().is_success() {
    //                        clog!("dashboard-request: success ok()");
    //                    } else {
    //                        clog!("dashboard-request: not found");
    //                    }
    //                }
    //                Err(err) => {
    //                    clog!("dashboard-request: failed request");
    //                }
    //            }
    //        });
    //    })
    //};

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
        //<button onclick={dashboard_request}>{"request dashboard"}</button>
        //<button onclick={oncheckuser}>{"check user"}</button>
        < Home />
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
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}
enum AuthAction {
    True,
    False,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct User {
    id: u32,
    username: String,
    token: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
struct UserState {
    has_user: Option<User>
}
impl Reducible for UserState {
    type Action = Option<User>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Some(s) => {
                UserState { has_user: Some(s) }.into()
            },
            None => UserState { has_user: None }.into()
        }
    }
}

//LINK - User Struct
#[derive(Clone, Debug, PartialEq, Properties)]
struct HasUser {
    user: Option<User>,
}
impl Default for HasUser {
    fn default() -> Self {
        Self { user: None }
    }
}
impl Reducible for HasUser {
    type Action = AuthAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::True => {
                HasUser { user: self.user.clone() }.into()
            },
            AuthAction::False => HasUser { user: None }.into()
        }
    }
}
#[derive(PartialEq, Properties)]
struct AuthProps {
    has_user_state: UseReducerHandle<HasUser>
}

#[function_component(AuthHeader)]
fn auth_header() -> Html {
    let ctx = use_context::<UserStateContext>().expect("no ctx AuthHeader");
    match ctx.has_user {
        Some(_) => {
            html! {
                <li><Link<Route> to={Route::EditorDashboard}>{"Editor Dashboard"}</Link<Route>></li>
            }
        },
        None => {
            html! {
                <li><Link<Route> to={Route::EditorLogin}>{"Editor Login"}</Link<Route>></li>
            }
        }
    }    
}
#[function_component(Home)]
fn home() -> Html {
    let has_user_state = use_context::<UserStateContext>().expect("no ctx Home");
    let onload = {
        let has_user_state = has_user_state.clone();
        Callback::from(move |_| {
            match has_user_state.has_user {
                Some(_) => clog!("hasuser"),
                None => clog!("nouser")
            }
        })  
    };
    let onclick = {
        let has_user_state = has_user_state.clone();
        Callback::from(move |e: MouseEvent| {
            match has_user_state.has_user {
                Some(_) => clog!("hasuser"),
                None => clog!("nouser")
            }
        })  
    };
    html! {
        <>
        <div onwaiting={onload.clone()}></div>
        <button onclick={onclick.clone()}>{ "check user from home" }</button>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct AppProp {
    user_auth: UseReducerHandle<HasUser>
}

type UserStateContext = UseReducerHandle<UserState>;

#[function_component(App)]
pub fn app() -> Html {
    let theme_ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });
    let fallback = html! {
        <li><Link<Route> to={Route::EditorLogin}>{"Editor Login"}</Link<Route>></li>
    };
    let has_user_state = use_reducer(|| HasUser::default());
    let user_state = use_reducer(|| UserState {has_user:None});

    
    html! {
        <ContextProvider<UserStateContext> context={user_state}>
        <div>
            <BrowserRouter>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                    <AuthHeader />
                </ul>
            </nav>
            <main>
                <Switch<Route> render={switch} />
            </main>
            </BrowserRouter>
        </div>
        </ContextProvider<UserStateContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
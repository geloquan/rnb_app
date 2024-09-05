use std::{borrow::Borrow, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use gloo_utils::document;
use http::{header, HeaderMap, StatusCode};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlInputElement, Node, Storage};
use yew::{
    function_component, html, prelude::*, suspense::use_future, Html, Properties
};
use yew_router::prelude::*;
use yew::suspense::SuspensionResult;
use yew::suspense::Suspension;
use reqwest::{header::COOKIE, Client, Url};
use wasm_bindgen_futures::spawn_local;
use gloo::{console::log as clog, events::EventListener, net::http::Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use regex::Regex;

mod entity;

use entity::*;

#[derive(Properties, PartialEq, Clone)]
struct SessionToken {
    value: String
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editor/login")]
    EditorLogin,
    #[at("/editor/dashboard")]
    EditorDashboard,
    
    #[at("/:code")]
    Code { code: String },
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
        },
        Route::Code {code} => html! {
            <Code code={code}/>
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
                let local_storage = web_sys::window()
                .and_then(|win| win.local_storage().ok())
                .and_then(|storage| storage)
                .expect("LocalStorage is not available");
                let user_json = serde_json::to_string(&user).expect("Failed to serialize user to JSON");
                local_storage
                    .set_item("user", &user_json)
                    .expect("Failed to set item in localStorage");
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
//LINK - EditorDashboard
#[function_component(EditorDashboard)]
fn editor_dashboard() -> Html {
    html! {
    <>
        <div>{"hello from dashboard"}</div>
    </>
    }
}
#[derive(Properties, PartialEq)]
struct CodeProp {
    code: String
}

//LINK - SvgContent Reducible
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
struct SvgContent {
    svg: Option<String>
}
impl Reducible for SvgContent {
    type Action = Option<String>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Some(s) => {
                SvgContent { svg: Some(s) }.into()
            },
            None => SvgContent { svg: None }.into()
        }
    }
}
#[derive(Properties, PartialEq)]
struct OptionProps {
    x: Option<HashMap<String, bool>>,
    y: Option<HashMap<String, bool>>
}
//LINK - Options
mod option;
#[function_component(Options)]
fn options() -> Html {
    let entity_ctx = use_context::<EntityContext>().expect("no svg ctx");
    
    html! {
    <>
        <option::Y />
        <option::X />
    </>
    }
}
mod theme;
//LINK - Code
#[function_component(Code)]
fn code(code: &CodeProp) -> Html {
    let user_ctx = use_context::<UserStateContext>().expect("no User ctx found");
    let entity_ctx = use_reducer(|| Entity::new() );
    let focus = use_state(|| theme::Focus {
        stroke: "#000000 !important".to_owned(),
    });
    let unfocus = use_state(|| theme::Unfocus {
        stroke: "none !important".to_owned(),
        fill: "none !important".to_owned()
    });
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
    <>    
        <ContextProvider<theme::Focus> context={(*focus).clone()}>
        <ContextProvider<theme::Unfocus> context={(*unfocus).clone()}>
        <ContextProvider<EntityContext> context={entity_ctx}>
            <Options/> 
            <SvgData code={code.code.clone()}/>
        </ContextProvider<EntityContext>>
        </ContextProvider<theme::Unfocus>>
        </ContextProvider<theme::Focus>>
    </>
    }
}

#[function_component(SvgData)]
fn svg_data(code: &CodeProp) -> Html {
    let code = code.code.clone();
    let entity_ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    if entity_ctx.name == "" {
        spawn_local(async move {
            let svg_req = reqwasm::http::Request::get(
                &format!("http://127.0.0.2:8081/{:?}", code)
            )
            .send()
            .await;
            
            
            match svg_req {
                Ok(response) => {
                    if response.status_text() == "OK" {
                        let body_text = response.text().await.expect("Failed to get body text");
                        let entity: Result<Entity, serde_json::Error>  = serde_json::from_str(&body_text);
                        
                        if let Ok(mut entity) = entity {
                            entity_ctx.dispatch(EntityCase::Init(Some(entity)));
                            entity_ctx.dispatch(EntityCase::ProduceOption);
                            entity_ctx.dispatch(EntityCase::Highlight("".to_string()));
                        }
                    }
                },
                Err(_) => {
                }
            }
        });
    } 

    let entity_ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    let borrowed = entity_ctx.svg_content.borrow();
    if let Some(ref svg) = *borrowed {
            let div: Element = document().create_element("div").unwrap();
            div.set_inner_html(svg);
            //let target: EventTarget = div.clone().dyn_into::<EventTarget>().unwrap();
            //let closure = Closure::wrap(Box::new(move |event: Event| {
            //    web_sys::console::log_1(&"Event triggered".into());
            //    
            //}) as Box<dyn FnMut(_)>);
//
            //let clicked = {
            //    Callback::from(move |event: Event| {
            //    })
            //};
            //
            //target
            //.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            //.unwrap();
//
            //closure.forget();

            let node: Node = div.into();
            return html! {
                Html::VRef(node)
            }
    } else {
        return html! {
            <div>
                {"upload svg"}
            </div>
        }
    }
    //match entity_ctx {
    //    Some(ref svg) => {
    //        let div: Element = document().create_element("div").unwrap();
    //        div.set_inner_html(svg);
    //        //let target: EventTarget = div.clone().dyn_into::<EventTarget>().unwrap();
    //        //let closure = Closure::wrap(Box::new(move |event: Event| {
    //        //    web_sys::console::log_1(&"Event triggered".into());
    //        //    
    //        //}) as Box<dyn FnMut(_)>);
////
    //        //let clicked = {
    //        //    Callback::from(move |event: Event| {
    //        //    })
    //        //};
    //        //
    //        //target
    //        //.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
    //        //.unwrap();
////
    //        //closure.forget();
//
    //        let node: Node = div.into();
    //        html! {
    //            Html::VRef(node)
    //        }
    //    },
    //    None => {
    //        html! {
    //            <div>
    //                {"upload svg"}
    //            </div>
    //        }
    //    }
    //}
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
            
            spawn_local(async move {
                let usere = login(&params_json).await;
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

fn app_load_state(local_storage_user: Option<String>, local_storage: Storage, has_user_ctx: UseReducerHandle<UserState>) -> Result<(), Error> {
    if let Some(user_data) = local_storage_user {
        let match_user_data: Result<Option<User>, serde_json::Error> = serde_json::from_str(&user_data);
        let user_state_contexta = has_user_ctx.clone();
        match match_user_data {
            Ok(data) => {
                let local_storage = local_storage.clone();
                spawn_local(async move {
                    let user_state_contextb = user_state_contexta.clone();
                    let url_str = "http://127.0.0.2:8081/editor/reauth";
                    let req = reqwasm::http::Request::post(url_str)
                    .header("content-type", "applcation/json")
                    .body(user_data)
                    .send()
                    .await;
                    match req {
                        Ok(response) => {
                            if response.status_text() == "OK" {
                                user_state_contextb.dispatch(data);
                            } else {
                                let _remove_loaded = local_storage.remove_item("user");
                                user_state_contextb.dispatch(None);
                            }
                        },
                        Err(_) => {
                            user_state_contextb.dispatch(None);
                        }
                    };
                });
            },
            Err(_) => {
                user_state_contexta.dispatch(None);
            },
        };
    };
    let app_state = AppState { loaded: true };
    local_storage
    .set_item("loaded", &serde_json::to_string(&app_state).unwrap())
    .expect("Failed to set item in localStorage");
    Ok(())
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
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct AppState {
    loaded: bool
}

type UserStateContext = UseReducerHandle<UserState>;
type EntityContext = UseReducerHandle<Entity>;
//type EditorContext = UseReducerHandle<Editor>;


#[function_component(App)]
pub fn app() -> Html {
    let local_storage = web_sys::window()
    .and_then(|win| win.local_storage().ok())
    .and_then(|storage| storage)
    .expect("LocalStorage is not available");
    let local_storage_user = local_storage
        .get_item("user")
        .expect("Failed to get item from localStorage");
    let local_storage_loaded = local_storage
        .get_item("loaded")
        .expect("Failed to get item from localStorage");
    let _remove_loaded = local_storage.remove_item("loaded");
    let user_state = use_reducer(|| UserState {has_user: None});
    
    let onload = {
        let local_storage = local_storage.clone();
        let local_storage_loaded = local_storage_loaded.clone();
        let has_user_ctx = user_state.clone();
        match local_storage_loaded {
            Some(loaded) => {
                let match_loaded_data: Result<AppState, serde_json::Error> = serde_json::from_str(&loaded);
                match match_loaded_data {
                    Ok(app_state) => {
                        match app_state.loaded {
                            true => {
                                clog!("true");
                            },
                            false => {
                                clog!("false");
                                let _ = app_load_state(local_storage_user, local_storage, has_user_ctx);
                            }
                        }
                    },
                    Err(e) => {
                    }
                }
            },
            None => {
                let _ = app_load_state(local_storage_user, local_storage, has_user_ctx);
            }
        }
        Callback::from(|_| {
            clog!("onload() callback");
        })
    };
    
    html! {
    <>
        <ContextProvider<UserStateContext> context={user_state}>
        <div {onload}>
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
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
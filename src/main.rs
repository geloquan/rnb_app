use core::borrow;
use std::{borrow::{Borrow, BorrowMut}, cell::{Cell, RefCell}, collections::HashMap, ops::Deref, rc::Rc};

use gloo_utils::document;
use http::{header, HeaderMap, StatusCode};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlElement, HtmlInputElement, Node, Storage};
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

mod svg;
use svg::*;

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

pub mod option;
use option::*;

//LINK - Options
#[function_component(Options)]
fn options() -> Html {
    let entity_ctx = use_context::<EntityContext>().expect("no svg ctx");
    
    html! {
    <>
        <option::y::Y />
        <option::x::X />
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
    let code: String = code.code.clone();
    let entity_ctx: UseReducerHandle<Entity> = use_context::<EntityContext>().expect("no Svg Content ctx found");
    if entity_ctx.name.borrow().is_empty() {
        spawn_local(async move {
            let svg_req: Result<reqwasm::http::Response, reqwasm::Error> = reqwasm::http::Request::get(
                &format!("http://127.0.0.2:8081/{:?}", code)
            )
            .send()
            .await;
            match svg_req {
                Ok(response) => {
                    if response.status_text() == "OK" {
                        let body_text = response.text().await.expect("Failed to get body text");
                        let entity: Result<EntityResponse, serde_json::Error>  = serde_json::from_str(&body_text);
                        
                        if let Ok(entity) = entity {
                            let ent = entity::Entity::to_entity(entity);
                            entity_ctx.dispatch(EntityCase::Init(Some(ent)));
                            entity_ctx.dispatch(EntityCase::ProduceOption(None));
                            entity_ctx.dispatch(EntityCase::Highlight("".to_string()));
                        }
                    }
                },
                Err(_) => {
                }
            }
        });
    } 
    
    let context = use_context::<EntityContext>().expect("no Svg Content ctx found");
    let svg_content = context.svg_content.borrow();
    let svg_content_highlighted = context.svg_content_highlighted.borrow();

    let div: Element = document().create_element("div").unwrap();
    let target: HtmlElement = div.clone().dyn_into::<HtmlElement>().unwrap();
    let is_dragging: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let offset_x: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let offset_y: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    let saved_left: Rc<Cell<i32>> = Rc::new(Cell::new(0)); // Initial left position
    let saved_top: Rc<Cell<i32>> = Rc::new(Cell::new(0));  // Initial top position

    {
        let is_dragging_clone: Rc<Cell<bool>> = is_dragging.clone();
        let offset_x_clone: Rc<Cell<i32>> = offset_x.clone();
        let offset_y_clone: Rc<Cell<i32>> = offset_y.clone();
        let saved_left_clone: Rc<Cell<i32>> = saved_left.clone();
        let saved_top_clone: Rc<Cell<i32>> = saved_top.clone();

        let on_mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
            is_dragging_clone.set(true);
    
            offset_x_clone.set(event.client_x() - saved_left_clone.get());
            offset_y_clone.set(event.client_y() - saved_top_clone.get());
        }) as Box<dyn FnMut(_)>);

            div.add_event_listener_with_callback("mousedown", on_mousedown.as_ref().unchecked_ref())
            .unwrap();
        on_mousedown.forget();
    }

    {
        let target = target.clone();
        let is_dragging_clone = is_dragging.clone();
        let offset_x_clone = offset_x.clone();
        let offset_y_clone = offset_y.clone();

        let on_mousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
            if is_dragging_clone.get() {
                let new_left = event.client_x() - offset_x_clone.get();
                let new_top = event.client_y() - offset_y_clone.get();
                target
                .set_attribute("style", 
                    &format!(
                        "
                            position: relative !important; 
                            overflow: hidden;
                            top: {}px;
                            left: {}px;
                            right: {}px;
                        ", 
                        new_top,
                        new_left,
                        -new_left 
                    )
                )
                .unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        div
            .add_event_listener_with_callback("mousemove", on_mousemove.as_ref().unchecked_ref())
            .unwrap();
        on_mousemove.forget();
    }

    {
        let target = target.clone();
        let is_dragging_clone = is_dragging.clone();
        let saved_left_clone = saved_left.clone();
        let saved_top_clone = saved_top.clone();

        let on_mouseup = Closure::wrap(Box::new(move |_event: MouseEvent| {
            is_dragging_clone.set(false);
            let current_left = target.get_attribute("style")
                .and_then(|style| {
                    style.split(';')
                        .find(|s| s.trim_start().starts_with("left:"))
                        .map(|s| s.replace("left:", "").replace("px", "").trim().parse::<i32>().unwrap())
                }).unwrap_or(0); 
            let current_top = target.get_attribute("style")
                .and_then(|style| {
                    style.split(';')
                        .find(|s| s.trim_start().starts_with("top:"))
                        .map(|s| s.replace("top:", "").replace("px", "").trim().parse::<i32>().unwrap())
                }).unwrap_or(0); 
            
            saved_left_clone.set(current_left);
            saved_top_clone.set(current_top);
        }) as Box<dyn FnMut(_)>);

        div
            .add_event_listener_with_callback("mouseup", on_mouseup.as_ref().unchecked_ref())
            .unwrap();
        on_mouseup.forget();
    }

    let svge = if let Some(svge) = &svg_content_highlighted.svg_content {
        svge
    } else if let Some(svge) = &svg_content.svg_content {
        svge
    } else {
        "upload svg"
    };

    div.set_inner_html(svge);

    let node: Node = div.into();
    
    return html! {
    <>
        {Html::VRef(node)}
    </>
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
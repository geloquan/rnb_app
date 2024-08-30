use std::{borrow::Borrow, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use gloo_utils::document;
use http::{header, HeaderMap, StatusCode};
use wasm_bindgen::JsValue;
use web_sys::{js_sys::{Error, JSON}, window, Element, HtmlInputElement, Node, Storage};
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
//LINK - Code
#[function_component(Code)]
fn code(code: &CodeProp) -> Html {
    let user_ctx = use_context::<UserStateContext>().expect("no User ctx found");
    let svg_ctx = use_reducer(|| SvgContent {svg: None});
    let fallback = html! {<div>{"Loading..."}</div>};
    let svg = {
        let user_ctx = user_ctx.clone();
        //let e = spawn_local(async move {
        //    let url_str = format!("http://127.0.0.2:8081/{:?}", code.code.clone());
        //    Ok(html! {
        //        <div> {code.code.clone()} </div>
        //    })
        //});
    };
    html! {
    <>
        <ContextProvider<SvgContentContext> context={svg_ctx}>
        //{svg_content}
        <SvgData code={code.code.clone()}/>
        <div> {svg} </div>
        </ContextProvider<SvgContentContext>>
    </>
    }
}
#[function_component(SvgData)]
fn svg_data(code: &CodeProp) -> Html {
    let code = code.code.clone();
    let svg_content_ctx = use_context::<SvgContentContext>().expect("no Svg Content ctx found");
    spawn_local(async move {
        let svg_req = reqwasm::http::Request::get(
            &format!("http://127.0.0.2:8081/{:?}", code)
        )
        .send()
        .await;
        //let svg_option_req = reqwasm::http::Request::get(
        //    &format!("http://127.0.0.2:8081/{:?}/option", code)
        //)
        //.send()
        //.await;
        //clog!(format!("svg_option_req: {:?}", svg_option_req));

        clog!(format!("svg_req: {:?}", svg_req));

        match svg_req {
            Ok(response) => {
                clog!("Ok(response)");
                if response.status_text() == "OK" {
                    clog!("Success to submit form");
                    let body_text = response.text().await.expect("Failed to get body text");

                    svg_content_ctx.dispatch(Some(body_text));
                    

                }
            },
            Err(_) => {
                clog!("Err(_)");
            }
        }
    });

    let svg_content_ctx2 = use_context::<SvgContentContext>().expect("no Svg Content ctx found");
    match &svg_content_ctx2.svg {
        Some(svg) => {
            let div: Element = document().create_element("div").unwrap();
            div.set_inner_html(svg);
            let node: Node = div.into();
            html! {
                Html::VRef(node)
            }
        },
        None => {
            html! {
                <div>
                    {"upload svg"}
                </div>
            }
        }
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

fn app_load_state(local_storage_user: Option<String>, local_storage: Storage, has_user_ctx: UseReducerHandle<UserState>) -> Result<(), Error> {
    clog!("None(loaded)");
    if let Some(user_data) = local_storage_user {
        let match_user_data: Result<Option<User>, serde_json::Error> = serde_json::from_str(&user_data);
        let user_state_contexta = has_user_ctx.clone();
        match match_user_data {
            Ok(data) => {
                clog!("Ok(data)");
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
                                clog!("OK");
                                
                                user_state_contextb.dispatch(data);
                            } else {
                                clog!("NOT OK");

                                let _remove_loaded = local_storage.remove_item("user");
                                user_state_contextb.dispatch(None);
                            }
                        },
                        Err(_) => {
                            clog!("Err(response)");
                            user_state_contextb.dispatch(None);
                        }
                    };
                });
            },
            Err(_) => {
                clog!("Err(data)");
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
type SvgContentContext = UseReducerHandle<SvgContent>;

//fn id_to_class(svg_element: &str) -> &str {
//    let g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
//    let floor_value = Regex::new(r#"floor\S*"#).unwrap();
//    let id_value = Regex::new(r#"data-name="([^"]+)""#).unwrap();
//    let shape_tag = Regex::new(r#"<(polygon|rect|path)\b[^>]*>(.*?)"#).unwrap();
//    let class_value = Regex::new(r#"class="([^"]+)""#).unwrap();
//
//    let mut modified_content = svg_element.to_string();
//
//    for some_g in g_tag.captures_iter(svg_element) {
//        if let Some(g) = some_g.get(0) {
//            for some_id_value in id_value.captures(g.as_str()) {
//                let raw_id_value = some_id_value.get(1).unwrap().as_str();
//                let id_vec_value = raw_id_value.split(' ').collect::<Vec<&str>>();
//                let floor = floor_value.captures(some_id_value.get(1).unwrap().as_str()).unwrap().get(0).unwrap().as_str();
//                self.join_classes(
//                    id_vec_value
//                        .clone()
//                        .into_iter()
//                        .map(|id| (floor.to_string(), id.to_string()))
//                        .collect::<Vec<(String, String)>>(),
//                );
//                for some_shape_tag in shape_tag.captures_iter(g.as_str()) {
//                    if let Some(shape_tag_value) = some_shape_tag.get(0) {
//                        for some_class_value in class_value.captures_iter(shape_tag_value.as_str()) {
//                            //println!("some_class_value {:?}", some_class_value);
//                            let class_full_match = some_class_value.get(0).expect("Full match capture is missing").as_str();
//                            let new_class_content = class_full_match.replace(&format!(r#"class="{}""#, some_class_value.get(1).unwrap().as_str()), &format!(r#"class="{} {}""#, some_class_value.get(1).unwrap().as_str(), raw_id_value));
//                            
//                            let new_shape_tag = shape_tag_value.as_str().replace(&format!(r#"class="{}""#, some_class_value.get(1).unwrap().as_str()), &new_class_content);
//                            let full_g_match = some_g.get(0).expect("Full match capture is missing").as_str();
//                            let new_g_2 = full_g_match.replace(shape_tag_value.as_str(), &new_shape_tag);
//                            modified_content = modified_content.replace(full_g_match, &new_g_2);
//                        }
//                    } 
//                }
//            }
//        }
//    }
//    
//    "hello"
//}

#[function_component(App)]
pub fn app() -> Html {
    clog!("start up");
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
        clog!("onload()");
        let local_storage = local_storage.clone();
        let local_storage_loaded = local_storage_loaded.clone();
        let has_user_ctx = user_state.clone();
        match local_storage_loaded {
            Some(loaded) => {
                clog!("Some(loaded)");
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
                clog!("None(loaded)");
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
//
//fn join_classes(&mut self, id_vec: Vec<(String, String)>) {fn join_classes(&mut self, id_vec: Vec<(String, String)>) {
//    if let Some(ref mut classes) = self.classes {    if let Some(ref mut classes) = self.classes {
//        for id in id_vec.into_iter() {        for id in id_vec.into_iter() {
//            classes.insert(id, true);            classes.insert(id, true);
//        }        }
//    } else {    } else {
//        let mut new_classes = HashMap::new();        let mut new_classes = HashMap::new();
//        for id in id_vec.into_iter() {        for id in id_vec.into_iter() {
//            new_classes.insert(id, true);            new_classes.insert(id, true);
//        }        }
//        self.classes = Some(new_classes);        self.classes = Some(new_classes);
//    }    }
//}}
//fn join_data_name(&mut self, id_vec: Vec<String>) {fn join_data_name(&mut self, id_vec: Vec<String>) {
//    if let Some(ref mut data_name) = self.data_name {    if let Some(ref mut data_name) = self.data_name {
//        for id in id_vec.into_iter() {        for id in id_vec.into_iter() {
//            data_name.insert(id, true);            data_name.insert(id, true);
//        }        }
//    } else {    } else {
//        let mut new_data_name = HashMap::new();        let mut new_data_name = HashMap::new();
//        for id in id_vec.into_iter() {        for id in id_vec.into_iter() {
//            new_data_name.insert(id, true);            new_data_name.insert(id, true);
//        }        }
//        self.data_name = Some(new_data_name);        self.data_name = Some(new_data_name);
//    }    }
//}}
//#[derive(Clone, Debug)]#[derive(Clone, Debug)]
//pub struct Entity {pub struct Entity {
//    name: String,    name: String,
//    pub svg_content: Option<String>,    pub svg_content: Option<String>,
//    pub filter_options: Option<Node>,    pub filter_options: Option<Node>,
//    pub classes: Option<HashMap<(String, String), bool>>,    pub classes: Option<HashMap<(String, String), bool>>,
//    pub data_name: Option<HashMap<String, bool>>,    pub data_name: Option<HashMap<String, bool>>,
//    pub filter_actions: Option<String>,    pub filter_actions: Option<String>,
//    pub edit_mode: bool,    pub edit_mode: bool,
//    pub default_floor: String    pub default_floor: String
//}}
//impl Entity {impl Entity {
//    pub fn new() -> Self {    pub fn new() -> Self {
//        println!("Entity new()");        println!("Entity new()");
//        Entity {        Entity {
//            name: "".to_string(),            name: "".to_string(),
//            svg_content: None,            svg_content: None,
//            filter_options: None,            filter_options: None,
//            classes: None,            classes: None,
//            data_name: None,            data_name: None,
//            filter_actions: None,            filter_actions: None,
//            edit_mode: false,            edit_mode: false,
//            default_floor: "".to_string(),            default_floor: "".to_string(),
//        }        }
//    }    }
//    fn option_element(&mut self) {    fn option_element(&mut self) {
//        let div: Element = document().create_element("div").unwrap();        let div: Element = document().create_element("div").unwrap();
//        let select_x: Element = document().create_element("select").unwrap();        let select_x: Element = document().create_element("select").unwrap();
//        let select_y: Element = document().create_element("select").unwrap();        let select_y: Element = document().create_element("select").unwrap();
//
//        let mut x_cache: Vec<&str> = Vec::new();        let mut x_cache: Vec<&str> = Vec::new();
//        let mut y_cache: Vec<&str> = Vec::new();        let mut y_cache: Vec<&str> = Vec::new();
//                
//        match &self.classes {        match &self.classes {
//            Some(classes) => {            Some(classes) => {
//                let mut y_items: Vec<String> = Vec::new();                let mut y_items: Vec<String> = Vec::new();
//                for (class, _) in classes {                for (class, _) in classes {
//                    let class_name = &class.0;                    let class_name = &class.0;
//                                        
//                    if class_name.contains("floor") && !y_cache.contains(&class_name.as_str()) {                    if class_name.contains("floor") && !y_cache.contains(&class_name.as_str()) {
//                        y_items.push(class_name.clone());                        y_items.push(class_name.clone());
//                        y_cache.push(class_name.as_str());                        y_cache.push(class_name.as_str());
//                    }                    }
//
//                    if class.0 == self.default_floor && !x_cache.contains(&class.1.as_str()) {                    if class.0 == self.default_floor && !x_cache.contains(&class.1.as_str()) {
//                        let option: Element = document().create_element("option").unwrap();                        let option: Element = document().create_element("option").unwrap();
//                        option.set_node_value(Some(class.1.as_str()));                        option.set_node_value(Some(class.1.as_str()));
//                        option.set_text_content(Some(class.1.as_str()));                        option.set_text_content(Some(class.1.as_str()));
//                        let option_node: Node = option.into();                        let option_node: Node = option.into();
//                        let _ = select_x.append_child(&option_node);                        let _ = select_x.append_child(&option_node);
//                        x_cache.push(&class.1.as_str());                        x_cache.push(&class.1.as_str());
//                    }                    }
//                }                }
//                                
//                y_items.sort();                y_items.sort();
//                for item in y_items {                for item in y_items {
//                    let option: Element = document().create_element("option").unwrap();                    let option: Element = document().create_element("option").unwrap();
//                    option.set_node_value(Some(item.as_str()));                    option.set_node_value(Some(item.as_str()));
//                    option.set_text_content(Some(item.as_str()));                    option.set_text_content(Some(item.as_str()));
//                    let option_node: Node = option.into();                    let option_node: Node = option.into();
//                    let _ = select_y.append_child(&option_node);                    let _ = select_y.append_child(&option_node);
//                }                }
//            },            },
//            None => {            None => {
//            }            }
//        }        }
//                
//        let node: Node = div.into();        let node: Node = div.into();
//        self.filter_options = Some(node);        self.filter_options = Some(node);
//    }    }
//}}
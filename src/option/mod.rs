use gloo::console::log as clog;
use gloo_utils::document;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, use_context, Event, Html};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlInputElement, Node, Storage};

use crate::{EntityContext, _Entity::default_floor, entity, Entity, EntityCase};

#[function_component(Y)]
pub fn y() -> Html {
    let ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    let select: Element = document().create_element("select").unwrap();
    if let Some(ref map) = &ctx.y_option {
        let mut sorted_vec: Vec<(&String, &String)> = map.iter().collect();
        sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
    }
    match (&ctx.y_option, &ctx.default_floor) {
        (Some(y), floor) if !floor.is_empty() => {
            for option_y in y {
                if option_y.1 == floor {
                    let option: Element = document().create_element("option").unwrap();
                    option.set_text_content(Some(&option_y.0));
                    option.set_attribute("value", &option_y.0);
                    select.append_child(&option);
                } 
            }
            
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
        (Some(y), _) => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
        (None, floor) if floor.is_empty() => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        }
        (_, _) => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        }
    }
}
#[function_component(X)]
pub fn x() -> Html {
    let ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    let select: Element = document().create_element("select").unwrap();
    let borrow = ctx.x_option.borrow();
    if let Some(ref map) = *borrow {
        let mut sorted_vec: Vec<(&String, &String)> = map.iter().collect();
        sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
        match (&ctx.default_floor) {
            (floor) if !floor.is_empty() => {
                for option_x in map {
                    if *option_x.1 == *floor {
                        let option: Element = document().create_element("option").unwrap();
                        option.set_text_content(Some(&option_x.0));
                        option.set_attribute("value", &option_x.0);
    
                        select.append_child(&option);
                    }
                }
                let target: EventTarget = select.clone().dyn_into::<EventTarget>().unwrap();
                let ctx_clone = ctx.clone(); 
                let closure = Closure::wrap(Box::new(move |event: Event| {
                    let ctx_clone = ctx_clone.clone(); 
                    let target = event.target().unwrap();
                    let val = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap().value();
                    ctx_clone.dispatch(EntityCase::Highlight(val));
                    
                }) as Box<dyn FnMut(_)>);
                    target
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
                
                let node: Node = select.into();
                html! {
                    Html::VRef(node)
                }
            },
            (_) => {
                let node: Node = select.into();
                html! {
                    Html::VRef(node)
                }
            },
        }
    } else {
        return html! {
            
        }
    }
}
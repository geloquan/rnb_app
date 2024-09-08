use std::fmt::format;

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

            for i in 0..5 {
                let option: Element = document().create_element("option").unwrap();
                option.set_text_content(Some(&i.to_string()));
                option.set_attribute("value", &i.to_string());
                select.append_child(&option);
            }
            
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
        (Some(y), _) => {
            for i in 0..5 {
                let option: Element = document().create_element("option").unwrap();
                option.set_text_content(Some(&i.to_string()));
                option.set_attribute("value", &i.to_string());
                select.append_child(&option);
            }
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
        (None, floor) if floor.is_empty() => {
            for i in 0..5 {
                let option: Element = document().create_element("option").unwrap();
                option.set_text_content(Some(&i.to_string()));
                option.set_attribute("value", &i.to_string());
                select.append_child(&option);
            }
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        }
        (_, _) => {
            for i in 0..5 {
                let option: Element = document().create_element("option").unwrap();
                option.set_text_content(Some(&i.to_string()));
                option.set_attribute("value", &i.to_string());
                select.append_child(&option);
            }
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
    match &borrow.data {
        Some(map) => {
                let mut sorted_vec: Vec<(&String, &String)> = map.iter().collect();
                sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
                match &ctx.default_floor {
                    floor if !floor.is_empty() => {
                        for option_x in map {
                            if *option_x.1 == *floor {
                                let option: Element = document().create_element("option").unwrap();
                                option.set_text_content(Some(&option_x.0));
                                let _ = option.set_attribute("value", &option_x.0);
            
                                let _ = select.append_child(&option);
                            }
                        }
        
                        let target: EventTarget = select.clone().dyn_into::<EventTarget>().unwrap();
                        let ctx_clone = ctx.clone(); 
                        let closure = Closure::wrap(Box::new(move |event: Event| {
                        // TODO -     let ctx_clone = ctx_clone.clone(); 
                        // TODO -     let target = event.target().unwrap();
                        // TODO -     let element = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                        // TODO -     let val = element.value();
                        // TODO -     ctx_clone.dispatch(EntityCase::Highlight(val));
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
        },
        None => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
    }
}
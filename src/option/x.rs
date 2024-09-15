use std::fmt::format;

use gloo::console::log as clog;
use gloo_utils::document;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, use_context, Event, Html};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlInputElement, Node, Storage};

use crate::{EntityContext, _Entity::default_floor, entity, Entity, EntityCase};

#[function_component(X)]
pub fn x() -> Html {
    let ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    let select: Element = document().create_element("select").unwrap();
    let borrow = ctx.x_option.borrow();
    let current_option_borrow = ctx.current_option.borrow();
    match &borrow.data {
        Some(map) => {
            let mut sorted_vec: Vec<(&String, &String)> = map.iter().collect();
            sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
            let floor = if let Some(y) = &current_option_borrow.y {
                y.as_str()
            } else if &ctx.default_floor != "" {
                &ctx.default_floor.as_str()
            } else {
                ""
            };
            match &current_option_borrow.x {
                Some(s) => {
                    let option: Element = document().create_element("option").unwrap();
                    option.set_text_content(Some(s));
                    let _ = option.set_attribute("value", s);
                    let _ = option.set_attribute("selected", "");
                    let _ = option.set_attribute("disabled", "");
                    let _ = select.append_child(&option);
                },
                None => {
                    let option: Element = document().create_element("option").unwrap();
                    option.set_text_content(Some("-- select slot --"));
                    let _ = option.set_attribute("value", "");
                    let _ = option.set_attribute("selected", "");
                    let _ = option.set_attribute("disabled", "");
                    let _ = select.append_child(&option);
                }
            }
            clog!(format!("map: {:?}", map));
            clog!(format!("map floor: {:?}", floor));
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
                let ctx_clone = ctx_clone.clone(); 
                let target = event.target().unwrap();
                let element = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                let val = element.value();
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
            //match &ctx.default_floor {
            //    floor if !floor.is_empty() => {
            //    },
            //    (_) => {
            //        let node: Node = select.into();
            //        html! {
            //            Html::VRef(node)
            //        }
            //    },
            //}
        },
        None => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
    }
}
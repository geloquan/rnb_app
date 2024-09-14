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
    let borrow = ctx.y_option.borrow();
    let current_option_borrow = ctx.current_option.borrow();
    match &borrow.data {
        Some(map) => {
            let mut sorted_vec: Vec<(&String, &String)> = map.iter().collect();
            sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
            let current_floor = &ctx.default_floor;
            match &current_option_borrow.y {
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
                    option.set_text_content(Some(&ctx.default_floor));
                    let _ = option.set_attribute("value", current_floor);
                    let _ = option.set_attribute("selected", "");
                    let _ = option.set_attribute("disabled", "");
                    let _ = select.append_child(&option);
                }
            }
            for option_y in sorted_vec {
                if option_y.0 != current_floor {
                } 
                let option: Element = document().create_element("option").unwrap();
                option.set_text_content(Some(&option_y.0));
                let _ = option.set_attribute("value", &option_y.0);

                let _ = select.append_child(&option);
            }

            let target: EventTarget = select.clone().dyn_into::<EventTarget>().unwrap();
            let ctx_clone = ctx.clone(); 
            let closure = Closure::wrap(Box::new(move |event: Event| {
                let ctx_clone = ctx_clone.clone(); 
                let target = event.target().unwrap();
                let element = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                let val = element.value();
                ctx_clone.dispatch(EntityCase::ProduceOption(Some(val)));
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
        None => {
            let node: Node = select.into();
            html! {
                Html::VRef(node)
            }
        },
    }
}

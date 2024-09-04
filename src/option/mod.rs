use gloo_utils::document;
use yew::{function_component, html, use_context, Html};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlInputElement, Node, Storage};

use crate::{EntityContext, _Entity::default_floor};

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
                let option: Element = document().create_element("option").unwrap();
                option.set_node_value(Some(&option_y.0));
                option.set_attribute("value", &option_y.0);
                select.append_child(&option);
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
    let _ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    html! {
    }
}
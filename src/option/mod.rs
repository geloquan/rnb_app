use yew::{function_component, html, use_context, Html};

use crate::{EntityContext, _Entity::default_floor};


#[function_component(Y)]
pub fn y() -> Html {
    let ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    match (&ctx.y_option, &ctx.default_floor) {
        (Some(y), floor) if !floor.is_empty() => {
            html! {
            }
        },
        (Some(y), _) => {
            html! {
            }
        },
        (_, _) => {
            html! {
            }
        }
    }
    //if let Some(y) = &ctx.y_option {
    //    html! {
    //    }
    //} else {
    //    html! {
    //    }
    //}
}
#[function_component(X)]
pub fn x() -> Html {
    let _ctx = use_context::<EntityContext>().expect("no Svg Content ctx found");
    html! {
    }
}
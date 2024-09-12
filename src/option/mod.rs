use std::fmt::format;

use gloo::console::log as clog;
use gloo_utils::document;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, use_context, Event, Html};
use web_sys::{js_sys::{self, Error, JSON}, window, Element, EventTarget, HtmlInputElement, Node, Storage};

use crate::{EntityContext, _Entity::default_floor, entity, Entity, EntityCase};

pub mod y;
pub mod x;

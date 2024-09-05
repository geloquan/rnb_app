use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{HashMap, HashSet}, ops::{Deref, Range}, rc::Rc, vec};

use _Entity::{data_name, default_floor};
use gloo::console::log as clog;
use gloo_utils::document;
use regex::Regex;
use serde::{Serialize, Deserialize};
use web_sys::Element;
use yew::{use_context, Properties, Reducible};

use crate::theme::Focus;

#[derive(Debug)]
pub enum EntityCase {
    //Code,
    //Editor,

    Init(Option<Entity>),
    Highlight(String),
    ProduceOption
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityResponse {
    pub name: String,
    pub svg_raw_content: Option<String>,
    pub svg_content: Option<String>,
    pub default_floor: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct Entity {
    pub name: String,
    pub svg_raw_content: Option<String>,
    pub svg_content: RefCell<Option<String>>,
    pub default_floor: String,
    pub current_floor: Option<String>,

    pub x_option: RefCell<Option<HashMap<String, String>>>,
    pub y_option: Option<HashMap<String, String>>,

    pub focus_option: Option<String>,
    
    pub classes: Option<HashMap<(String, String), bool>>,
    pub data_name: Option<HashMap<String, bool>>,
}
impl Reducible for Entity {
    type Action = EntityCase;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EntityCase::Init(init) => {
                match init {
                    Some(s) => {
                        s.into()
                    },
                    None => Entity::new().into()
                }
            },
            EntityCase::Highlight(highlight) => {
                let _ = self.highlight_option(Some(&highlight));
                self.clone()
            },
            EntityCase::ProduceOption => {
                let _ = self.produce_option(None);
                self.clone()
            }
        } 
    }
}
impl Entity {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            svg_raw_content: None,
            svg_content: None.into(),
            default_floor: "".to_string(),
            current_floor: None,
            x_option: None.into(),
            y_option: None,
            focus_option: None,
            classes: None,
            data_name: None,
        }
    }
    pub fn get_all(&self) -> Self {
        self.clone()
    }
    fn has_all_values(&self) -> bool {
        self.name != "" && 
        self.svg_raw_content.is_some() && 
        self.svg_content.borrow().is_some() &&
        self.default_floor != ""
    }
    
    pub fn produce_option(& self, floor: Option<&str>) -> Result<Self, &'static str> {
        let _g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let floor_value = Regex::new(r#"floor\S*"#).unwrap();
        let data_name_property = Regex::new(r#"data-name="([^"]+)""#).unwrap();
        let _shape_tag = Regex::new(r#"<(polygon|rect|path)\b[^>]*>(.*?)"#).unwrap();
        let _class_value = Regex::new(r#"class="([^"]+)""#).unwrap();

        let focus_style = r#"style="stroke: #000000 !important""#;
        let unfocus_style = r#"style="stroke: none !important; fill: none !important""#;

        let mut _x: HashMap<String, bool> = HashMap::new();

        let mut ranges: Vec<Range<i32>> = Vec::new();
            
        let mut to_focus_ranges: Vec<Range<i32>> = Vec::new();
        
        let mut svg_raw_content = self.svg_raw_content.clone();

        if let Some(svg_raw_content) = &svg_raw_content {
            for some_data_name_property in data_name_property.captures_iter(&svg_raw_content) {
                if let Some(data_name_property) = some_data_name_property.get(0) {
                    let raw_range = some_data_name_property.get(0).unwrap();
                    let start = raw_range.start() as i32;
                    let end = raw_range.end() as i32;

                    let data_name_properties = some_data_name_property
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .collect::<Vec<&str>>();

                    if data_name_properties.contains(&self.default_floor.as_str()) ||
                    data_name_properties.contains(&floor.unwrap_or("")) 
                    {
                        to_focus_ranges.push(start..end);
                    }

                    ranges.push(start..end);

                    let equal_floor: bool = data_name_properties.iter().any(|data_name_value| {
                        data_name_value == &floor.unwrap_or("") ||
                        data_name_value == &self.default_floor
                    });


                    if equal_floor {
                        for data_name_value in data_name_properties.iter() {
                            if data_name_value.contains("floor-") {
                                continue;
                            } else {
                                let mut borrow = self.x_option.borrow_mut();
                                match (borrow.clone(), self.default_floor.clone(), floor) {
                                    (Some(mut vec), _, Some(floor)) => {
                                        vec.insert(data_name_value.to_string(), floor.to_string());
                                    },
                                    (Some(mut vec), default_floor_, None) => {
                                        vec.insert(data_name_value.to_string(), default_floor_.to_string());
                                    },
                                    (None, default_floor_, _) => {
                                        let mut temp = HashMap::new(); 
                                        temp.insert(data_name_value.to_string(), default_floor_.to_string());
                                        borrow.replace(temp);
                                    }
    
                                }
                            }
                        }
                    } 
                    //for data_name_value in data_name_properties.iter() {
                    //    if data_name_value.contains("floor-") {
                    //        continue;
                    //    } else {
                    //        clog!(format!("data_name_value {:?}", data_name_value));
                    //        
                    //    }
                    //    //match &mut self.y_option {
                    //    //    Some(vec) => {
                    //    //        for data_name_value_2 in data_name_properties.clone() {
                    //    //            vec.insert(data_name_value_2.to_string(), data_name_value.to_string());
                    //    //        }
                    //    //    }
                    //    //    None => {
                    //    //        let mut  temp = HashMap::new(); 
                    //    //        for data_name_value_2 in data_name_properties.clone() {
                    //    //            temp.insert(data_name_value_2.to_string(), data_name_value.to_string());
                    //    //        }
                    //    //        self.y_option = Some(temp);
                    //    //    }
                    //    //}
                    //}
                }
            }
        }
        
        {
            let mut unique_ranges = HashSet::new();
            let mut unique_ranges_vec: Vec<Range<i32>> = ranges
                .clone()
                .into_iter()
                .filter(|range| unique_ranges.insert((range.start, range.end)))
                .collect();
            let mut to_focus_unique_ranges = HashSet::new();
            let mut to_focus_unique_ranges_vec: Vec<Range<i32>> = to_focus_ranges
                .clone()
                .into_iter()
                .filter(|range| to_focus_unique_ranges.insert((range.start, range.end)))
                .collect();
    
            if let Some(ref mut svg_raw_content) = svg_raw_content {
                while let Some(last_element) = unique_ranges_vec.last() {
                    if to_focus_unique_ranges_vec.contains(last_element) {
                        svg_raw_content.insert_str((last_element.end).try_into().unwrap(), focus_style);
                    } else {
                        svg_raw_content.insert_str((last_element.end).try_into().unwrap(), unfocus_style);
                    }
        
                    unique_ranges_vec.pop();
                } 
            }
        }

        self.svg_content.borrow_mut().replace(svg_raw_content.unwrap_or("".to_string()));
        
        Ok(self.to_owned())
    }

    fn g_tag_parse() {
        
    }

    pub fn highlight_option(& self, slot: Option<&str>) -> Result<Self, &'static str> {
        clog!("highlight_option");

        if slot.is_none() { return Err("nothing to process") }
        else if slot.unwrap() == self.focus_option.clone().unwrap_or("".to_string()) { return Err("nothing to process") }
        //self.svg_content = None;
        //self.svg_content.borrow_mut().replace("".to_string());

        let g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let shape_tag = Regex::new(r#"<(polygon)\b[^>]*>(.*?)"#).unwrap();
        let data_name_property = Regex::new(r#"data-name="([^"]+)""#).unwrap();
        let focus_style = r#"style="stroke: #000000 !important""#;
        let highlight_style = r#"style="stroke: red !important""#;
        
        let floor_scope = if let Some(current_floor) = &self.current_floor {
            current_floor
        } else {
            &self.default_floor
        };
        
        //let mut svg_content_clone: String = String::new();
        
        self.svg_content.borrow_mut().replace("ep".to_owned());
        let borrowed = self.svg_content.borrow();
        let binding = self.svg_content.clone();
        
        if let Some(ref svg_content) = *borrowed {
            //let capture: Vec<usize> = svg_content.match_indices(focus_style).map(|(index, _)| index + focus_style.len()).collect();
            //clog!(format!("capture: {:?}", capture));
            //for some_g_tag_element in g_tag.captures_iter(&svg_content) {
            //    clog!(format!("some_g_tag_element: {:?}", some_g_tag_element));
            //    if let Some(g_tag_element) = some_g_tag_element.get(0) {
            //        let g_tag_element_start = g_tag_element.start();
            //        let g_tag_element_end = g_tag_element.end();
            //        let g_tag_element_value = g_tag_element.as_str();
            //        for some_data_name_property in data_name_property.captures_iter(&g_tag_element.as_str()) {
            //            clog!(format!("some_data_name_property: {:?}", some_data_name_property));
            //            if let Some(data_name_property_value) = some_data_name_property.get(1) {
            //                clog!(format!("data_name_property_value: {:?}", data_name_property_value.as_str()));
//
            //                let data_name_properties = some_data_name_property
            //                .get(1)
            //                .unwrap()
            //                .as_str()
            //                .split(' ')
            //                .collect::<Vec<&str>>();
            //                clog!(format!("data_name_properties: {:?}", data_name_properties));
            //                
            //                let equal_slot: bool = if data_name_properties.contains(&slot.unwrap_or("")) &&
            //                data_name_properties.contains(&floor_scope.as_str()) {
            //                    true
            //                } else {
            //                    false
            //                };
//
            //                if equal_slot {
            //                    for some_shape_tag in shape_tag.captures_iter(&g_tag_element.as_str()) {
            //                        clog!(format!("qwq: some_shape_tag: {:?}", some_shape_tag));
            //                        if let (Some(polygon_element), Some(shape_element_tag_name)) = (some_shape_tag.get(0), some_shape_tag.get(1)) {
            //                            clog!(format!("qwq: polygon_element: {:?}", polygon_element));
            //                            clog!(format!("qwq: shape_element_tag_name: {:?}", shape_element_tag_name));
            //                            let polygon_element_start = polygon_element.start();
            //                            let polygon_element_end =   polygon_element.end();
            //                            let polygon_element_value = polygon_element.as_str();
//
            //                            let shape_element_tag_name_start = shape_element_tag_name.start();
            //                            let shape_element_tag_name_end =   shape_element_tag_name.end();
            //                            let shape_element_tag_name_value = shape_element_tag_name.as_str();
            //                            match shape_element_tag_name_value {
            //                                "polygon" => {
            //                                    clog!("polygon");
            //                                    let res = polygon_element_value.replace(focus_style, &highlight_style);
            //                                    clog!(format!("qwq: res: {:?}", res));
            //                                    
            //                                    let mut g_tag_element_value = g_tag_element_value.to_string();
            //                                    
            //                                    g_tag_element_value.replace_range(polygon_element_start..polygon_element_end, &res);
            //                                    clog!(format!("qwq: g_tag_element_value: {:?}", g_tag_element_value));
            //                                    
            //                                    clog!(format!("qwq: svg_content_clone: {:?}", svg_content_clone));
            //                                    &svg_content_clone.replace_range(g_tag_element_start..g_tag_element_end, &g_tag_element_value);
//
            //                                    clog!("OH");
            //                                },
            //                                "g" => {
            //                                    clog!("g");
            //                                },
            //                                _ => {
            //                                    clog!("_");
            //                                }
            //                            }
            //                        }
            //                    }
            //                }
            //            }
            //        }
            //    }
            //}
            //self.svg_content.borrow_mut().replace(svg_content_clone);
            clog!(format!("qwq: self.svg_content: {:?}", self.svg_content));
            //for some_data_name_property in data_name_property.captures_iter(&svg_content) {
            //    if let Some(data_name_property) = some_data_name_property.get(0) {
            //        clog!(format!("some_data_name_property: {:?}", some_data_name_property));
            //        let raw_range = some_data_name_property.get(0).unwrap();
            //        let end = raw_range.end() as i32;
            //        clog!(format!("end: {:?}", end));
            //        
            //        let data_name_properties = some_data_name_property
            //        .get(1)
            //        .unwrap()
            //        .as_str()
            //        .split(' ')
            //        .collect::<Vec<&str>>();
//
            //        let equal_slot: bool = data_name_properties.iter().any(|data_name_value| {
            //            data_name_value == &slot.unwrap_or("") 
            //        });
//
            //        if equal_slot {
            //            let _ = capture.clone().into_iter().map(|index| {
            //                clog!(format!("index as i32 - end: {:?}", index as i32 - end));
            //                if (0 <= (index as i32 - end)) && ((index as i32 - end) <= 50) {
            //                    clog!("hello");
            //                }
            //            }).collect::<Vec<_>>();
            //        }
            //    }
            //}
        } else {

        }

        //self.svg_content.borrow_mut().replace(svg_content.unwrap_or("".to_string()));
        
        Ok(self.to_owned())
    }
    
    //pub fn build_svg(&self, entity_case: EntityCase) -> Result<(), &'static str> {
    //    if self.has_all_values() { return Err("one of the property may be empty"); }
    //    match entity_case {
    //        EntityCase::Code => {
//
    //            Ok(())
    //        }
    //        EntityCase::Editor => {
    //            Ok(())
    //        },
    //    }
    //}
}
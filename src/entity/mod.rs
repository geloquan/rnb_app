use std::{collections::{HashMap, HashSet}, ops::Range, rc::Rc};

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
    Code,
    Editor,
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
    pub svg_content: Option<String>,
    pub default_floor: String,
    pub current_floor: Option<String>,

    pub x_option: Option<HashMap<String, String>>,
    pub y_option: Option<HashMap<String, String>>,
    
    pub classes: Option<HashMap<(String, String), bool>>,
    pub data_name: Option<HashMap<String, bool>>,
}
impl Reducible for Entity {
    type Action = Option<Entity>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Some(s) => {
                s.into()
            },
            None => Entity::new().into()
        }
    }
}
impl Entity {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            svg_raw_content: None,
            svg_content: None,
            default_floor: "".to_string(),
            current_floor: None,
            x_option: None,
            y_option: None,
            classes: None,
            data_name: None,
        }
    }
    fn has_all_values(&self) -> bool {
        self.name != "" && 
        self.svg_raw_content.is_some() && 
        self.svg_content.is_some() &&
        self.default_floor != ""
    }
    
    pub fn produce_option(&mut self, floor: Option<&str>) -> Result<Self, &'static str> {
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

                    clog!(format!("data_name_properties {:?}", data_name_properties));

                    if equal_floor {
                        clog!("equal_floor");
                        for data_name_value in data_name_properties.iter() {
                            if data_name_value.contains("floor-") {
                                continue;
                            } else {
                                match (&mut self.x_option, self.default_floor.clone(), floor) {
                                    (Some(vec), _, Some(floor)) => {
                                        clog!("Some(vec)");
                                        for data_name_value_2 in data_name_properties.iter() {
                                            clog!(format!("data_name_value_2 {:?}", data_name_value_2));
                                        }
                                        vec.insert(data_name_value.to_string(), floor.to_string());
                                    },
                                    (Some(vec), default_floor_, None) => {
                                        vec.insert(data_name_value.to_string(), default_floor_.to_string());
                                    },
                                    (None, default_floor_, _) => {
                                        let mut temp = HashMap::new(); 
                                        temp.insert(data_name_value.to_string(), default_floor_.to_string());
                                        self.x_option = Some(temp);
                                    }
    
                                }
                            }
                        }
                    } else {
                        clog!("else");
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

        self.svg_content = svg_raw_content.clone();
        
        Ok(self.to_owned())
    }
    
    pub fn build_svg(&self, entity_case: EntityCase) -> Result<(), &'static str> {
        if self.has_all_values() { return Err("one of the property may be empty"); }
        match entity_case {
            EntityCase::Code => {

                Ok(())
            }
            EntityCase::Editor => {
                Ok(())
            },
        }
    }
}
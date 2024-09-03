use std::{collections::{HashMap, HashSet}, ops::Range, rc::Rc};

use _Entity::data_name;
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

    pub x_option: Option<HashMap<String, bool>>,
    pub y_option: Option<HashMap<String, bool>>,
    
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
    
    pub fn produce_option(&mut self) -> Result<Self, &'static str> {
        let _g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let _floor_value = Regex::new(r#"floor\S*"#).unwrap();
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
                    clog!(format!("Some(data_name_property): {:?}", Some(data_name_property)));
                    let raw_range = some_data_name_property.get(0).unwrap();
                    let start = raw_range.start() as i32;
                    let end = raw_range.end() as i32;

                    let data_name_properties = some_data_name_property
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .collect::<Vec<&str>>();
                    clog!(format!("id_vec_value: {:?}", data_name_properties));

                    if data_name_properties.contains(&self.default_floor.as_str()) ||
                    data_name_properties.contains(&self.current_floor.clone().unwrap_or("".to_owned()).as_str()) 
                    {
                        to_focus_ranges.push(start..end);
                    }
                    ranges.push(start..end);

                    //ranges.sort_by(|a, b| b.end.cmp(&a.end));

                    //for some_id_value in id_value.captures(g.as_str()) {
                    //    let raw_range = some_id_value.get(0).unwrap();
                    //    let start = raw_range.start() as i32;
                    //    let end = raw_range.end() as i32;
                    //    ranges.push(start..end);
                    //    ranges.sort_by(|a, b| b.end.cmp(&a.end));
                    //    let mut unique_ranges = HashSet::new();
                    //    let unique_ranges_vec: Vec<Range<i32>> = ranges
                    //        .clone()
                    //        .into_iter()
                    //        .filter(|range| unique_ranges.insert((range.start, range.end)))
                    //        .collect();
                    //    let raw_id_value = some_id_value.get(1).unwrap().as_str();
                    //    let id_vec_value = raw_id_value.split(' ').collect::<Vec<&str>>();
                    //    let floor = floor_value.captures(some_id_value.get(1).unwrap().as_str()).unwrap().get(0).unwrap().as_str();
                    //    
                    //    clog!(format!("some_id_value: {:?}", some_id_value));
                    //    clog!(format!("ranges: {:?}", unique_ranges_vec));
                    //    
                    //    clog!(format!("raw_id_value: {:?}", raw_id_value));
                    //    clog!(format!("id_vec_value: {:?}", id_vec_value));
                    //    clog!(format!("floor: {:?}", floor));
                    //    clog!(format!("g: {:?}", g));
                    //    if let Some(current_floor) = &self.current_floor {
                    //        if current_floor == floor {
                    //            for id in id_vec_value {
                    //                x.insert(id.to_string(), true); 
                    //            }
                    //        }
                    //    } else if self.default_floor == floor {
                    //        for id in id_vec_value {
                    //            x.insert(id.to_string(), true); 
                    //        }
                    //        let option: Element = document().create_element("option").unwrap();
                    //        //for some_shape_tag in shape_tag.captures_iter(g.as_str()) {
                    //        //    if let Some(shape_tag_value) = some_shape_tag.get(0) {
                    //        //        for some_class_value in class_value.captures_iter(shape_tag_value.as_str()) {
                    //        //            //println!("some_class_value {:?}", some_class_value);
                    //        //            let class_full_match = some_class_value.get(0).expect("Full match capture is missing").as_str();
                    //        //            let new_class_content = class_full_match.replace(&format!(r#"class="{}""#, some_class_value.get(1).unwrap().as_str()), &format!(r#"class="{} {}""#, some_class_value.get(1).unwrap().as_str(), raw_id_value));
                    //        //            
                    //        //            let new_shape_tag = shape_tag_value.as_str().replace(&format!(r#"class="{}""#, some_class_value.get(1).unwrap().as_str()), &new_class_content);
                    //        //            let full_g_match = some_g.get(0).expect("Full match capture is missing").as_str();
                    //        //            let new_g_2 = full_g_match.replace(shape_tag_value.as_str(), &new_shape_tag);
                    //        //            
                    //        //            clog!(format!("class_full_match: {:?}", class_full_match));
                    //        //            clog!(format!("new_class_content: {:?}", new_class_content));
                    //        //            clog!(format!("new_shape_tag: {:?}", new_shape_tag));
                    //        //            clog!(format!("full_g_match: {:?}", full_g_match));
                    //        //            clog!(format!("new_g_2: {:?}", new_g_2));
                    //        //        }
                    //        //    } 
                    //        //}
                    //    }
                    //}
                }
            }
        }
        
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

        clog!(format!("self.current_floor: {:?}", self.current_floor));
        clog!(format!("self.default_floor: {:?}", self.default_floor));
        clog!(format!("ranges: {:?}", unique_ranges_vec));
        clog!(format!("to_focus_unique_ranges_vec: {:?}", to_focus_unique_ranges_vec));
        if let Some(ref mut svg_raw_content) = svg_raw_content {
            while let Some(last_element) = unique_ranges_vec.last() {
                if to_focus_unique_ranges_vec.contains(last_element) {
                    clog!(format!("Last element: {}", last_element.end));
                    svg_raw_content.insert_str((last_element.end).try_into().unwrap(), focus_style);
                } else {
                    svg_raw_content.insert_str((last_element.end).try_into().unwrap(), unfocus_style);
                }
    
                unique_ranges_vec.pop();
            } 
        }

        clog!(format!("ranges: {:?}", unique_ranges_vec));

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
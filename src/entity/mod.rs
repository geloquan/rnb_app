use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{HashMap, HashSet}, ops::{Deref, DerefMut, Range}, rc::Rc, vec};

use _Entity::{data_name, default_floor};
use gloo::console::log as clog;
use gloo_utils::document;
use regex::Regex;
use serde::{Serialize, Deserialize};
use web_sys::Element;
use yew::{use_context, Properties, Reducible};

use crate::{_SvgContent::svg, theme::Focus, BuildNestedElement, NestedElement};

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
pub struct SvgContentt {
    pub svg_content: Option<String>
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct Entity {
    pub name: RefCell<String>,
    pub svg_raw_content: RefCell<Option<String>>,
    pub svg_content: RefCell<SvgContentt>,
    pub svg_content_app: Option<String>,
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
    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EntityCase::Init(init) => {
                match init {
                    Some(s) => {
                        return s.into()
                    },
                    None => return Entity::new().into()
                }
            },
            EntityCase::Highlight(highlight) => {
                clog!(format!("highlight: {:?}", highlight));
                let new_string = self.clone().highlight_option(Some(&highlight));
                match new_string {
                    Ok(string) => {
                        let mut svg_content = Entity::mutate(&mut self);
                        svg_content.svg_content = Some(string);
                    },
                    Err(e) => {
                        clog!(e);
                    }
                }
            },
            EntityCase::ProduceOption => {
                let new_string = self.clone().produce_option(None).unwrap();
                let mut svg_content = Entity::mutate(&mut self);
                svg_content.svg_content = Some(new_string);
            }
        } 
        self
    }
}
impl Entity {
    fn mutate<'a>(self: &'a mut Rc<Self>) -> impl 'a + DerefMut<Target = SvgContentt> {
        let this = Rc::make_mut(self);
        this.svg_content.borrow_mut()
    }
    pub fn replacee(entity_response: EntityResponse) -> Self {
        Self {
            name: entity_response.name.into(),
            svg_raw_content: entity_response.svg_raw_content.into(),
            svg_content: SvgContentt {svg_content: None}.into(),
            svg_content_app: None,
            default_floor: entity_response.default_floor,
            current_floor: None,
            x_option: None.into(),
            y_option: None,
            focus_option: None,
            classes: None,
            data_name: None,
        }
    }
    pub fn new() -> Self {
        Self {
            name: "".to_string().into(),
            svg_raw_content: None.into(),
            svg_content: SvgContentt {svg_content: None}.into(),
            svg_content_app: None,
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
        self.name.borrow().is_empty() && 
        self.svg_raw_content.borrow().is_some() && 
        //self.svg_content.borrow().is_some() &&
        self.default_floor != ""
    }
    pub fn produce_option(& self, floor: Option<&str>) -> Result<String, &'static str> {
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
        
        let mut svg_raw_content = self.svg_raw_content.borrow().clone();

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
                                    (Some(_), _, Some(floor)) => {
                                        borrow.as_mut().unwrap().insert(data_name_value.to_string(), floor.to_string());
                                    },
                                    (Some(_), default_floor_, None) => {
                                        borrow.as_mut().unwrap().insert(data_name_value.to_string(), default_floor_.to_string());
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
        //self.svg_content.borrow_mut().replace(svg_raw_content.clone().unwrap_or("".to_string()));
        Ok(svg_raw_content.clone().unwrap_or("".to_string()))
    }

    pub fn highlight_option(& self, slot: Option<&str>) -> Result<String, &'static str> {
        clog!("highlight_option");

        if slot.is_none() { return Err("nothing to process") }
        else if slot.unwrap() == self.focus_option.clone().unwrap_or("".to_string()) { return Err("nothing to process") }
        //self.svg_content = None;
        //self.svg_content.borrow_mut().replace("".to_string());

        let g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let shape_tag = Regex::new(r#"<(polygon)\b[^>]*>(.*?)"#).unwrap();
        let data_name_property = Regex::new(r#"data-name="([^"]+)""#).unwrap();
        
        let focus_style = r#"style="stroke: #000000 !important""#;

        let mut nest: BuildNestedElement = BuildNestedElement {
            nests: Vec::new(),
            svg_content: String::new()
        };
        
        let floor_scope = if let Some(current_floor) = &self.current_floor {
            current_floor
        } else {
            &self.default_floor
        };
        
        let svg_content = &self.svg_content.borrow().svg_content;
        let mut svg_content_clone = String::new();
        if let Some(svg_content) = svg_content {
            let capture: Vec<usize> = svg_content.match_indices(focus_style).map(|(index, _)| index + focus_style.len()).collect();
            svg_content_clone = svg_content.clone().to_string();
            nest.svg_content = svg_content.clone().to_string();
            for some_g_tag_element in g_tag.captures_iter(&svg_content) {
                if let Some(g_tag_element) = some_g_tag_element.get(0) {
                    let g_tag_element_start = g_tag_element.start();
                    let g_tag_element_end = g_tag_element.end();
                    let g_tag_element_value = g_tag_element.as_str();
                    for some_data_name_property in data_name_property.captures_iter(&g_tag_element.as_str()) {
                        if let Some(data_name_property_value) = some_data_name_property.get(1) {

                            let data_name_properties = some_data_name_property
                            .get(1)
                            .unwrap()
                            .as_str()
                            .split(' ')
                            .collect::<Vec<&str>>();
                            clog!(format!("data_name_properties: {:?}", data_name_properties));
                            
                            let equal_slot: bool = if data_name_properties.contains(&slot.unwrap_or("")) &&
                            data_name_properties.contains(&floor_scope.as_str()) {
                                true
                            } else {
                                false
                            };

                            if equal_slot {
                                for some_shape_tag in shape_tag.captures_iter(&g_tag_element.as_str()) {
                                    if let (Some(polygon_element), Some(shape_element_tag_name)) = (some_shape_tag.get(0), some_shape_tag.get(1)) {
                                        let polygon_element_start = polygon_element.start();
                                        let polygon_element_end =   polygon_element.end();
                                        let polygon_element_value = polygon_element.as_str();

                                        let shape_element_tag_name_value = shape_element_tag_name.as_str();

                                        nest.nests.push(NestedElement {
                                            polygon_element_start,
                                            polygon_element_end,
                                            polygon_element_value: polygon_element_value.to_owned(),
                                            
                                            g_tag_element_start,
                                            g_tag_element_end,
                                            g_tag_element_value: g_tag_element_value.to_owned(),
                                            
                                            shape_element_tag_name_value: shape_element_tag_name_value.to_owned(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {

        }
        let rr = nest.build();
        
        Ok(rr)
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
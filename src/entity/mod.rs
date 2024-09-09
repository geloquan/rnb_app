use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{HashMap, HashSet}, ops::{Deref, DerefMut, Range}, rc::Rc, vec};

use _Entity::{data_name, default_floor};
use gloo::console::log as clog;
use gloo_utils::document;
use regex::Regex;
use serde::{Serialize, Deserialize};
use web_sys::Element;
use yew::{use_context, Properties, Reducible};

use crate::{_SvgContent::svg, theme::Focus, BuildNestedElement, EntityContext, NestedElement};

#[derive(Debug)]
pub enum EntityCase {
    //Code,
    //Editor,

    Init(Option<Entity>),
    Hydrate,
    Highlight(String),
    ProduceOption(Option<String>),
    Floor(String)
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
pub struct OptionXY {
    pub x: Option<String>,
    pub y: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct OptionX {
    pub data: Option<HashMap<String, String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct OptionY {
    pub data: Option<HashMap<String, String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct Entity {
    pub name: RefCell<String>,
    pub svg_raw_content: RefCell<Option<String>>,
    pub svg_content: RefCell<SvgContentt>,
    pub svg_content_app: Option<String>,
    pub default_floor: String,
    pub current_option: RefCell<OptionXY>,

    pub x_option: RefCell<OptionX>,
    pub y_option: RefCell<OptionY>,

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
            EntityCase::Hydrate => {
                let y_option = self.clone().produce_option_y().unwrap();
                let mut option = Entity::mutate_option(&mut self);
                option.y = Some(y_option);
            },
            EntityCase::Highlight(highlight) => {
                let new_string = self.clone().highlight_option(Some(&highlight));
                match new_string {
                    Ok(string) => {
                        let mut tuple = Entity::mutate_to_highlight(&mut self);
                        tuple.0.svg_content = Some(string.clone());
                        tuple.1.x = Some(highlight.clone());
                    },
                    Err(e) => {
                        clog!(e);
                    }
                }
            },
            EntityCase::ProduceOption(floor_str) => {
                let new = self.clone().produce_option(floor_str).unwrap();
                let new_string = new.0;
                let new_option_x = new.1;
                let new_option_y = new.1;

                let mut svg_content = Entity::mutate_produce_option(&mut self);
                svg_content.0.svg_content = Some(new_string);
                svg_content.1.data = Some(new_option_x);
                svg_content.2.data = Some(new_option_y);
            },
        } 
        self
    }
}
impl Entity {
    fn mutate_to_highlight<'a>(self: &'a mut Rc<Self>) -> (impl 'a + DerefMut<Target = SvgContentt>, impl 'a + DerefMut<Target = OptionXY>) {
        let this = Rc::make_mut(self);
        (this.svg_content.borrow_mut(), this.current_option.borrow_mut())
    }

    fn mutate_floor<'a>(self: &'a mut Rc<Self>) -> impl 'a + DerefMut<Target = OptionXY> {
        let this = Rc::make_mut(self);
        this.current_option.borrow_mut()
    }

    fn mutate_svg_content<'a>(self: &'a mut Rc<Self>) -> impl 'a + DerefMut<Target = SvgContentt> {
        let this = Rc::make_mut(self);
        this.svg_content.borrow_mut()
    }
    
    fn mutate_produce_option<'a>(self: &'a mut Rc<Self>) -> (impl 'a + DerefMut<Target = SvgContentt>, impl 'a + DerefMut<Target = OptionX>, impl 'a + DerefMut<Target = OptionY>) {
        let this = Rc::make_mut(self);
        (this.svg_content.borrow_mut(), this.x_option.borrow_mut(), this.y_option.borrow_mut())
    }

    fn mutate_option_x<'a>(self: &'a mut Rc<Self>) -> impl 'a + DerefMut<Target = OptionX> {
        let this = Rc::make_mut(self);
        this.x_option.borrow_mut()
    }

    fn mutate_option<'a>(self: &'a mut Rc<Self>) -> impl 'a + DerefMut<Target = OptionXY> {
        let this = Rc::make_mut(self);
        this.current_option.borrow_mut()
    }

    pub fn to_entity(entity_response: EntityResponse) -> Self {
        Self {
            name: entity_response.name.into(),
            svg_raw_content: entity_response.svg_raw_content.into(),
            svg_content: SvgContentt {svg_content: None}.into(),
            svg_content_app: None,
            default_floor: entity_response.default_floor,
            current_option: OptionXY {x: None, y: None}.into(),
            x_option: OptionX {data: None}.into(),
            y_option: OptionY {data: None}.into(),
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
            current_option: OptionXY {x: None, y: None}.into(),
            x_option: OptionX {data: None}.into(),
            y_option: OptionY {data: None}.into(),
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
    pub fn produce_option_y(&self) -> Result<HashMap<String, String>, &'static str> {
        let mut option = HashMap::new();
        
        Ok(option)
    }
    pub fn produce_option(& self, floor: Option<String>) -> Result<(String, HashMap<String, String>, HashMap<String, String>), &'static str> {
        clog!("produce_option");
        let _g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let floor_value = Regex::new(r#"floor\S*"#).unwrap();
        let data_name_property = Regex::new(r#"data-name="([^"]+)""#).unwrap();
        let _shape_tag = Regex::new(r#"<(polygon|rect|path)\b[^>]*>(.*?)"#).unwrap();
        let _class_value = Regex::new(r#"class="([^"]+)""#).unwrap();

        let focus_style = r#"style="stroke: #000000 !important""#;
        let unfocus_style = r#"style="stroke: none !important; fill: none !important""#;

        let mut x: HashMap<String, String> = HashMap::new();
        let mut y: HashMap<String, String> = HashMap::new();

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
                                y.insert(data_name_value.to_string(), data_name_value.to_string());
                            } else {
                                let mut borrow = self.x_option.borrow();
                                match (self.default_floor.clone(), floor) {
                                    (_, Some(floor)) => {
                                        x.insert(data_name_value.to_string(), floor.to_string());
                                    },
                                    (default_floor_, None) => {
                                        x.insert(data_name_value.to_string(), default_floor_.to_string());
                                    },
                                    (default_floor_, _) => {
                                        x.insert(data_name_value.to_string(), default_floor_.to_string());
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
        Ok((svg_raw_content.clone().unwrap_or("".to_string()), x.to_owned(), y.to_owned()))
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
        let current_y = self.current_option.borrow().y.clone();
        
        let floor_scope = if let Some(current_y) = current_y {
            current_y
        } else {
            self.default_floor.clone()
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
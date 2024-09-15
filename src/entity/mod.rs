use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{HashMap, HashSet}, ops::{Deref, DerefMut, Range}, rc::Rc, vec};

use gloo::console::log as clog;
use gloo_utils::document;
use regex::Regex;
use serde::{Serialize, Deserialize};
use web_sys::Element;
use yew::{use_context, Properties, Reducible};

use crate::{_SvgContent::svg, entity, theme::Focus, BuildNestedElement, EntityContext, NestedElement};

#[derive(Debug)]
pub enum EntityCase {
    //Code,
    //Editor,

    Init(Option<Entity>),
    Highlight(String),
    ProduceOption(Option<String>),
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
pub struct ElementData {
    pub data: Option<HashMap<(String, String, Range<i32>), bool>>,
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
    
    pub element: RefCell<ElementData>,
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
                let new_option_y = new.2;
                let new_slots = new.3;

                let mut svg_content = Entity::mutate_produce_option(&mut self);
                svg_content.0.svg_content = Some(new_string);
                svg_content.1.data = Some(new_option_x);
                svg_content.2.data = Some(new_option_y);
                svg_content.3.data = Some(new_slots);
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
    
    fn mutate_produce_option<'a>(self: &'a mut Rc<Self>) -> (impl 'a + DerefMut<Target = SvgContentt>, impl 'a + DerefMut<Target = OptionX>, impl 'a + DerefMut<Target = OptionY>, impl 'a + DerefMut<Target = ElementData>) {
        let this = Rc::make_mut(self);
        (this.svg_content.borrow_mut(), this.x_option.borrow_mut(), this.y_option.borrow_mut(), this.element.borrow_mut())
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
            element: ElementData {data: None}.into(),
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
            element: ElementData {data: None}.into(),
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
    fn process_string(input: &str) -> String {
        let re = Regex::new(r"-(\d+)(?:(_[^_]+)?|-[^-_]+)").unwrap();
        let mut found_number = false;
        
        re.replace_all(input, |caps: &regex::Captures| {
            if let Some(_) = caps.get(1) {
                // Check if the number has already been found
                if !found_number {
                    found_number = true;
                    format!("-{}", &caps[1])  // Retain the first -[number]
                } else {
                    "".to_string()  // Remove any subsequent matches
                }
            } else if let Some(_) = caps.get(2) {
                "-_".to_string()  // Retain the -_ if present
            } else {
                "".to_string()  // Remove anything else
            }
        }).to_string()
    }
    pub fn produce_option(& self, floor: Option<String>) -> Result<(String, HashMap<String, String>, HashMap<String, String>, HashMap<(String, String, Range<i32>), bool>), &'static str> {
        clog!("produce_option");
        let _g_tag = Regex::new(r#"<g\b[^>]*>(.*?)<\/g>|<polygon\b[^>]*>(.*?)<\/polygon>|<g\b[^>]*\/>|<polygon\b[^>]*\/>"#).unwrap();
        let floor_value = Regex::new(r#"floor\S*"#).unwrap();
        let data_name_property = Regex::new(r#"data-name="([^"]+)""#).unwrap();
        let _shape_tag = Regex::new(r#"<(polygon|rect|path)\b[^>]*>(.*?)"#).unwrap();
        let _class_value = Regex::new(r#"class="([^"]+)""#).unwrap();
        
        let floor_value = Regex::new(r#"floor\S*"#).unwrap();
        let data_name_property = Regex::new(r#"id="([^"]+)""#).unwrap();

        let g_element = Regex::new(r#"<\s*>\s*<g[^>]*>"#).unwrap();
        
        let focus_style = r#"style="stroke: blue !important""#;
        let unfocus_style = r#"style="stroke: none !important; fill: none !important""#;

        let mut x: HashMap<String, String> = HashMap::new();
        let mut y: HashMap<String, String> = HashMap::new();
        
        // (id, data-name, style-range)
        let mut element: HashMap<(String, String, Range<i32>), bool> = HashMap::new();
        element.extend(self.element.borrow().data.clone().unwrap_or(HashMap::new()));

        let mut ranges: Vec<Range<i32>> = Vec::new();
        let mut to_focus_ranges: Vec<Range<i32>> = Vec::new();
        
        //let mut svg_raw_content = self.svg_raw_content.borrow().clone();
        let mut content: Option<String> = if let Some(svg_content_data) = self.svg_content.borrow().clone().svg_content {
            Some(svg_content_data)
        } else if let Some(svg_raw_content) = self.svg_raw_content.borrow().clone() {
            Some(svg_raw_content)
        } else {
            None
        };
        let current_option = self.current_option.borrow().clone();

        let floor: String = floor.clone().unwrap_or("".to_string());

        let floor_ref: &str = floor.as_str();
        
        let floor = if &floor_ref != &"" {
            &floor_ref
        } else if &self.default_floor.as_str() != &"" {
            &self.default_floor.as_str()
        } else {
            ""
        };

        clog!(format!("floor: {:?}", floor));

        if let Some(svg_raw_content) = &content {
            for some_data_name_property in data_name_property.captures_iter(&svg_raw_content) {
                if let (Some(data_name_property), Some(data_name_value)) = (some_data_name_property.get(0), some_data_name_property.get(1)) {
                    
                    let raw_range = some_data_name_property.get(0).unwrap();
                    let start = raw_range.start() as i32;
                    let end = raw_range.end() as i32;

                    let data_name_properties_vec = some_data_name_property
                    .get(1)
                    .unwrap()
                    .as_str(); 
                    
                    let processed_string = entity::Entity::process_string(data_name_properties_vec);

                    let data_name_properties = processed_string
                    .split('_') 
                    .collect::<Vec<&str>>();
                    
                    clog!(format!("data_name_property: {:?}", data_name_property));
                    clog!(format!("data_name_value: {:?}", data_name_value));
                    clog!(format!("data_name_properties: {:?}", data_name_properties));

                    if data_name_properties.contains(&floor) {
                        to_focus_ranges.push(start..end);
                    };

                    ranges.push(start..end);

                    for data_name_property in &data_name_properties {
                        element.insert((data_name_value.as_str().to_string(), data_name_property.to_string(), start..end), true);
                    }

                    let equal_floor: bool = data_name_properties.iter().any(|data_name_value| {
                        data_name_value == &floor.clone() ||
                        data_name_value == &self.default_floor
                    });

                    for data_name_value in data_name_properties.iter() {
                        if data_name_value.contains("floor-") {
                            y.insert(data_name_value.to_string(), data_name_value.to_string());
                        }
                    }

                    if equal_floor {
                        for data_name_value in data_name_properties.iter() {
                            let mut borrow = self.x_option.borrow();
                            match (self.default_floor.clone(), &floor_ref) {
                                (_, floor) if !floor.is_empty() => {
                                    x.insert(data_name_value.to_string(), floor.to_string());
                                },
                                (default_floor_, floor) if !floor.is_empty() => {
                                    x.insert(data_name_value.to_string(), default_floor_.to_string());
                                },
                                (default_floor_, _) => {
                                    x.insert(data_name_value.to_string(), default_floor_.to_string());
                                }

                            }
                        }
                    } 
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
            unique_ranges_vec.sort_by(|a, b| b.start.cmp(&a.start));

            let mut to_focus_unique_ranges = HashSet::new();
            let to_focus_unique_ranges_vec: Vec<Range<i32>> = to_focus_ranges
                .clone()
                .into_iter()
                .filter(|range| to_focus_unique_ranges.insert((range.start, range.end)))
                .collect();
            
            
            let mut sorted_elements: Vec<_> = element.clone().into_iter().collect();
            sorted_elements.sort_by(|a, b| a.0 .2.end.cmp(&b.0 .2.end));
            if let Some(ref mut svg_raw_content) = content {
                for unique_ranges_vece in &unique_ranges_vec {
                    let closing_bracket: Option<usize> = svg_raw_content.chars().skip(unique_ranges_vece.end as usize).position(|c| c == '>').map(|pos| pos + unique_ranges_vece.end as usize);
                    if let Some(closing_bracket) = closing_bracket {
                        clog!(format!("closing_bracket0: {:?}", closing_bracket));
                        if unique_ranges_vece.end as usize <= closing_bracket && closing_bracket <= svg_raw_content.len() {
                            clog!(format!("closing_bracket1: {:?}", closing_bracket));
                            let substring = &svg_raw_content[unique_ranges_vece.end as usize..closing_bracket];
                            if let Some(pos) = substring.find(focus_style) {
                                let replace_start = unique_ranges_vece.end as usize + pos;
                                let replace_end = replace_start + focus_style.len();
                                
                                if to_focus_unique_ranges_vec.iter().any(|to_range| {
                                    to_range.end == unique_ranges_vece.end
                                }){
                                    svg_raw_content.replace_range(replace_start..replace_end, focus_style);
                                } else {
                                    svg_raw_content.replace_range(replace_start..replace_end, unfocus_style);
                                }
                            } else if let Some(pos) = substring.find(unfocus_style) {
                                let replace_start = unique_ranges_vece.end as usize + pos;
                                let replace_end = replace_start + focus_style.len();
                                
                                if to_focus_unique_ranges_vec.iter().any(|to_range| {
                                    to_range.end == unique_ranges_vece.end
                                }){
                                    svg_raw_content.replace_range(replace_start..replace_end, focus_style);
                                } else {
                                    svg_raw_content.replace_range(replace_start..replace_end, unfocus_style);
                                }
                            } else {
                                if to_focus_unique_ranges_vec.iter().any(|to_range| {
                                    to_range.end == unique_ranges_vece.end
                                })
                                {
                                    svg_raw_content.insert_str((unique_ranges_vece.end).try_into().unwrap(), focus_style);
                                } else {
                                    svg_raw_content.insert_str((unique_ranges_vece.end).try_into().unwrap(), unfocus_style);
                                }
                            }
                        }
                    }
                }
                while let Some(ele) = sorted_elements.last() {
                    //if to_focus_unique_ranges_vec.iter().any(|range| {
                    //    if let Some(substring) = svg_raw_content.get(range.start as usize..range.end as usize) {
                    //        clog!("Substring: {}", substring);
                    //        if let Some(pos) = substring.find(focus_style) {
                    //            let replace_start = range.start as usize + pos;
                    //            let replace_end = replace_start + focus_style.len();
                    //            
                    //            svg_raw_content.replace_range(replace_start..replace_end, unfocus_style);
                    //        }
                    //    };
                    //    range.contains(&(ele.0.2.end as i32)) ||
                    //    range.end == ele.0.2.end as i32
                    //}) {
                    //    svg_raw_content.insert_str((ele.0.2.end).try_into().unwrap(), focus_style);
                    //} else {
                    //    svg_raw_content.insert_str((ele.0.2.end).try_into().unwrap(), unfocus_style);
                    //}
                    sorted_elements.pop();
                }
                    //if to_focus_unique_ranges_vec.contains(last_element) {
                    //    svg_raw_content.insert_str((last_element.end).try_into().unwrap(), focus_style);
                    //} else {
                    //    svg_raw_content.insert_str((last_element.end).try_into().unwrap(), unfocus_style);
                    //}
            }
        }

        Ok((content.clone().unwrap_or("".to_string()), x.to_owned(), y.to_owned(), element.to_owned()))
    }

    pub fn highlight_option(& self, slot: Option<&str>) -> Result<String, &'static str> {
        clog!("highlight_option");

        if slot.is_none() { return Err("nothing to process") }
        else if slot.unwrap() == self.focus_option.clone().unwrap_or("".to_string()) { return Err("nothing to process") }
        
        clog!(format!("slot: {:?}", slot));
        
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
}
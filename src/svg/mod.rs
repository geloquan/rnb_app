use gloo::console::log as clog;
use serde::{Serialize, Deserialize};
use yew::{Properties};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct NestedElement {
    pub polygon_element_start: usize,
    pub polygon_element_end: usize,
    pub polygon_element_value: String,

    pub g_tag_element_start: usize,
    pub g_tag_element_end: usize,
    pub g_tag_element_value: String,

    pub shape_element_tag_name_value: String,

}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct BuildNestedElement {
    pub nests: Vec<NestedElement>,
    pub svg_content: String
}
impl BuildNestedElement {
    pub fn build(&mut self) -> String {
        let focus_style = r#"style="stroke: #000000 !important""#;
        let highlight_style = r#"style="stroke: red !important""#;
        let mut svg_content_clone = self.svg_content.clone();

        let mut sorted_nests = self.nests.clone();
        sorted_nests.sort_by(|a, b| b.g_tag_element_end.cmp(&a.g_tag_element_end));

        for nested_element in &sorted_nests {
            clog!(format!("nested: {:?}", nested_element));
            match nested_element.shape_element_tag_name_value.as_str() {
                "polygon" => {
                    clog!("polygon");
                    if !nested_element.polygon_element_value.contains(focus_style) {
                        clog!("polygon if !");
                        let mut polygon_element_value = nested_element.polygon_element_value.to_string();
                        clog!(format!("qwq: polygon_element_value: {:?}", nested_element.polygon_element_value));
                        clog!(format!("qwq: polygon_element_start: {:?}", nested_element.polygon_element_start));
                        polygon_element_value.insert_str(9, &highlight_style);
                        clog!(format!("qwq: polygon_element_value: {:?}", polygon_element_value));
                        let mut g_tag_element_value = nested_element.g_tag_element_value.to_string();
                        let _ = &g_tag_element_value.replace_range(nested_element.polygon_element_start..nested_element.polygon_element_end, &polygon_element_value);
                        clog!(format!("qwq: g_tag_element_value: {:?}", g_tag_element_value));
    
                        let _ = &svg_content_clone.replace_range(nested_element.g_tag_element_start..nested_element.g_tag_element_end, &g_tag_element_value);
    
                        clog!("OH");
                    } else {
                        clog!("polygon else");
                        let res = nested_element.polygon_element_value.replace(focus_style, &highlight_style);
                        clog!(format!("qwq: res: {:?}", res));
                        
                        let mut g_tag_element_value = nested_element.g_tag_element_value.to_string();
                        
                        g_tag_element_value.replace_range(nested_element.polygon_element_start..nested_element.polygon_element_end, &res);
                        clog!(format!("qwq: g_tag_element_value: {:?}", g_tag_element_value));
                        
                        let _ = &svg_content_clone.replace_range(nested_element.g_tag_element_start..nested_element.g_tag_element_end, &g_tag_element_value);
    
                        clog!("OH");
                    }
                },
                "g" => {
                    clog!("g");
                },
                _ => {
                    clog!("_");
                }
            }
        }
        svg_content_clone
    }
}
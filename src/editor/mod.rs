struct ClassProperty {
    data_name: String,
    idle_icon_dir: Option<String>,
    hover_icon_dir: Option<String>,
    direct_to: String,
    sub_properties: Vec<(String, String)>,
}

struct Editor {
    alias_name: String,
    complete_name: String,
    file: Option<String>,
    properties: Option<Vec<ClassProperty>>, 

    public_gallery: Option<Vec<String>>,
    private_gallery: Option<Vec<String>>,
} 
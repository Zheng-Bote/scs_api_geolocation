use std::fs;

pub fn read_file(file_path: &str) -> String {
    let data: String = fs::read_to_string(file_path).expect("Unable to read file");
    return data;
}

pub fn add_component3(component_tag: String) -> String {
    let html_path =
        String::from("./templates/html/") + &component_tag.to_lowercase() + &String::from(".html");
    let html_loaded = read_file(&html_path);

    return html_loaded;
}

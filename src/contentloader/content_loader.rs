use std::fs;

pub fn read_file(file_path: &str) -> String {
    let data: String = fs::read_to_string(file_path).expect("Unable to read file");
    return data;
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag: String = component_tag.to_uppercase() + &String::from("_CSS");
    let html_tag: String = component_tag.to_uppercase() + &String::from("_HTML");
    let js_tag: String = component_tag.to_uppercase() + &String::from("_JS");

    let css_path =
        String::from("./templates/css/") + &component_tag.to_lowercase() + &String::from(".css");
    let css_loaded = read_file(&css_path);

    let html_path =
        String::from("./templates/html/") + &component_tag.to_lowercase() + &String::from(".html");
    let html_loaded = read_file(&html_path);

    let js_path =
        String::from("./templates/js/") + &component_tag.to_lowercase() + &String::from(".js");
    let js_loaded = read_file(&js_path);

    let html_data = html_data.replace(html_tag.as_str(), &html_loaded);
    let html_data = html_data.replace(css_tag.as_str(), &css_loaded);
    let html_data = html_data.replace(js_tag.as_str(), &js_loaded);
    return html_data;
}

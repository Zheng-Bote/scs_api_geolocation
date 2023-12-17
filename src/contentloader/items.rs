use super::content_loader::add_component;
use super::content_loader::read_file;

pub async fn items(content_type: String) -> String {
    let mut html_data = read_file("./templates/html/base.html");

    let js_base: String = read_file("./templates/js/base.js");
    let css_base: String = read_file("./templates/css/base.css");
    html_data = html_data.replace("{{BASE_CSS}}", &css_base);
    html_data = html_data.replace("{{BASE_JS}}", &js_base);
    html_data = add_component(content_type, html_data);

    return html_data;
}

pub async fn items2(content_type: String) -> [String; 3] {
    let mut html_data = read_file("./templates/html/base.html");

    let js_base: String = read_file("./templates/js/base.js");
    let css_base: String = read_file("./templates/css/base.css");
    //html_data = html_data.replace("{{BASE_CSS}}", &css_base);
    //html_data = html_data.replace("{{BASE_JS}}", &js_base);
    html_data = add_component(content_type, html_data);

    let arr = [html_data, css_base, js_base];
    return (arr);
}

mod content_loader;
mod items;

pub async fn get_filecontent3(content_type: String) -> String {
    let html_data: String = items::items3(content_type).await;
    return html_data;
}

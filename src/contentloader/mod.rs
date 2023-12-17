mod content_loader;
mod items;

pub async fn get_filecontent(content_type: String) -> String {
    let html_data: String = items::items(content_type).await;
    return html_data;
}

pub async fn get_filecontent2(content_type: String) -> [String; 3] {
    let html_data: [String; 3] = items::items2(content_type).await;
    return html_data;
}

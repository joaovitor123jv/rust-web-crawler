
pub fn get_page(page_url: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(&page_url)?.text()?)
}

pub fn extract_links(tokens: Vec<String>) -> Vec<String> {
    let mut links: Vec<String> = [].to_vec();

    for (index, token) in tokens.iter().enumerate() {
        if token == "href=" {
            if tokens[index + 1].starts_with("http") {
                links.push(tokens[index + 1].to_string());
            }
        }
    }

    links
}

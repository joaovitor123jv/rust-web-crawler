
pub fn get_page(page_url: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(&page_url)?.text()?)
}

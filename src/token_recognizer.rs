pub fn extract_tokens(whole_page: String) -> Vec<String> {
    whole_page
        .replace("<", " ")
        .replace(">", " ")
        .replace("/", " ")
        .replace("\"", " ")
        .replace("\'", " ")
        .split_whitespace() 
        .map(|s| s.to_string())
        .collect()
}

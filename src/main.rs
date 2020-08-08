mod lib;
mod email_recognizer;
mod token_recognizer;

fn main() {
    let mut links: Vec<String> = ["http://uma_url_aqui.com.br".to_string()].to_vec();
    let mut emails: Vec<String> = Vec::new();
    let mut iteration = 0;

    let emails_limit = 33;

    while emails.len() < emails_limit {
        let html_page = lib::web::get_page(links[iteration].clone());

        let html_page = match html_page {
            Ok(page) => page,
            Err(_error) => "".to_string()
        };

        if html_page != "".to_string() {
            println!("\nGetting html from url = {}", links[iteration]);
            let tokens = token_recognizer::extract_tokens(html_page);

            let page_emails = email_recognizer::extract_emails(tokens.clone());

            println!("\tPage emails: {:?}", page_emails);

            for link in lib::web::extract_links(tokens.clone()) {
                links.push(link.to_string());
            }

            for email in page_emails {
                emails.push(email);
            }
        }

        iteration += 1;
    }

    println!("\n\n---------------------------------");
    println!("Emails encontrados = {:?}", emails);
    println!("\nQuantidade de Links encontrados = {}", links.len());
    println!("Quantidade de páginas web buscadas = {}", iteration + 1);


    // let links: Vec<String> = lib::web::extract_page_links(html_page);
}


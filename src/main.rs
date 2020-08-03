mod lib;
mod email_recognizer;
mod token_recognizer;

fn test_chain(chain: &str, dfa: &lib::DFA) {
    println!("O resultado do teste da cadeia {} foi {}", chain, dfa.test(chain));
}


fn main() {
    println!("Testando importação de funcoes");

    let dfa = email_recognizer::create_email_recognizer().unwrap();

    // dfa.show();

    let html_page = lib::web::get_page("https://url_da_pagina.com".to_string()).unwrap();
 
    let tokens = token_recognizer::extract_tokens(html_page);

    for token in tokens {
        if dfa.test(&token) {
            println!("===== Achoou um email: {}", token);
        } 
        // else {
        //     println!("Não email: {}", token);
        // }
    }
}


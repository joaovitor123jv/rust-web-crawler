mod lib;

fn test_chain(chain: &str, dfa: &lib::DFA) {
    println!("O resultado do teste da cadeia {} foi {}", chain, dfa.test(chain).unwrap());
}


fn main() {
    println!("Testando importação de funcoes");

    let mut dfa = lib::DFA::new(
        "user", 
        "user_type", 
        "0123456789abcdefghijklmnopqrstuvwxyz@.", 
        &["user", "at", "provider", "user_type", "trash"]
    ).unwrap();

    dfa.add_transition("user", "user", '0').unwrap();
    dfa.add_transition("user", "user", '1').unwrap();
    dfa.add_transition("user", "user", '2').unwrap();
    dfa.add_transition("user", "user", '3').unwrap();
    dfa.add_transition("user", "user", '4').unwrap();
    dfa.add_transition("user", "user", '5').unwrap();
    dfa.add_transition("user", "user", '6').unwrap();
    dfa.add_transition("user", "user", '7').unwrap();
    dfa.add_transition("user", "user", '8').unwrap();
    dfa.add_transition("user", "user", '9').unwrap();


    dfa.add_transition("user", "user", 'z').unwrap();
    dfa.add_transition("user", "user", 'y').unwrap();
    dfa.add_transition("user", "user", 'x').unwrap();
    dfa.add_transition("user", "user", 'w').unwrap();
    dfa.add_transition("user", "user", 'v').unwrap();
    dfa.add_transition("user", "user", 'u').unwrap();
    dfa.add_transition("user", "user", 't').unwrap();
    dfa.add_transition("user", "user", 's').unwrap();
    dfa.add_transition("user", "user", 'r').unwrap();
    dfa.add_transition("user", "user", 'q').unwrap();
    dfa.add_transition("user", "user", 'p').unwrap();
    dfa.add_transition("user", "user", 'o').unwrap();
    dfa.add_transition("user", "user", 'n').unwrap();
    dfa.add_transition("user", "user", 'm').unwrap();
    dfa.add_transition("user", "user", 'l').unwrap();
    dfa.add_transition("user", "user", 'k').unwrap();
    dfa.add_transition("user", "user", 'j').unwrap();
    dfa.add_transition("user", "user", 'i').unwrap();
    dfa.add_transition("user", "user", 'h').unwrap();
    dfa.add_transition("user", "user", 'g').unwrap();
    dfa.add_transition("user", "user", 'f').unwrap();
    dfa.add_transition("user", "user", 'e').unwrap();
    dfa.add_transition("user", "user", 'd').unwrap();
    dfa.add_transition("user", "user", 'c').unwrap();
    dfa.add_transition("user", "user", 'b').unwrap();
    dfa.add_transition("user", "user", 'a').unwrap();



    dfa.add_transition("Q2", "Q4", '1').unwrap();
    dfa.add_transition("Q2", "Q1", '0').unwrap();

    dfa.add_transition("Q3", "Q4", '1').unwrap();
    dfa.add_transition("Q3", "Q1", '0').unwrap();

    dfa.add_transition("Q4", "Q1", '1').unwrap();
    dfa.add_transition("Q4", "Q3", '0').unwrap();

    dfa.show();

    // Aceita
    // Rejeita
    
    test_chain("01", &dfa);
    test_chain("1001", &dfa);
    test_chain("10011", &dfa);
    test_chain("", &dfa);
    test_chain("0", &dfa);
    test_chain("101010100101010010101001", &dfa);
}


mod lib;

fn test_chain(chain: &str, dfa: &lib::DFA) {
    println!("O resultado do teste da cadeia {} foi {}", chain, dfa.test(chain));
}

fn main() {
    println!("Testando importação de funcoes");

    let mut dfa = lib::DFA::new("Q1", "Q4", "01", &["Q1", "Q2", "Q3", "Q4"]).unwrap();

    dfa.add_transition("Q1", "Q2", '1').unwrap();
    dfa.add_transition("Q1", "Q3", '0').unwrap();

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
    test_chain("1010100101010101", &dfa);
}


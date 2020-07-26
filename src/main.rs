mod lib;

fn main() {
    println!("Testando importação de funcoes");

    let dfa = lib::DFA::new("Q1", "Q2", "aaaaaaabcdef", &["Q1", "Q2", "Q3"]).unwrap();

    dfa.show();
}




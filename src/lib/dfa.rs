use std::collections::HashSet;

// Deterministic Finite Automaton
pub struct DFA {
    initial_state: String,
    final_state: String,
    alphabet: HashSet<char>,
    states: HashSet<String> // Conjunto de Estados
    // transitions: ??? // Função de transição
}

impl DFA {
    // Construtor
    pub fn new(initial_state: &str, final_state: &str, alphabet: &str, states: &[&str]) -> Result<DFA, String> {
        let mut obtained_states = HashSet::new();

        for element in states.iter() {
            obtained_states.insert(element.to_string());
        }

        if !states.contains(&initial_state) {
            return Err("O estado inicial precisa estar presente no conjunto de estados do automato".to_string());
        }

        if !states.contains(&final_state) {
            return Err("O estado final precisa estar presente no conjunto de estados do automato".to_string());
        }

        let mut obtained_alphabet = HashSet::new();

        for symbol in alphabet.chars() {
            obtained_alphabet.insert(symbol);
        }

        Ok(DFA {
            initial_state: initial_state.to_string(),
            final_state: final_state.to_string(),
            alphabet: obtained_alphabet,
            states: obtained_states
        })
    }

    pub fn show(&self) {
        println!("O estado inicial é: {}, O estado final é: {}", self.initial_state, self.final_state);

        println!("\nOs estados do DFA são:");

        for state in &self.states {
            print!(" {} ", state);
        }


        println!("O alfabeto é composto pelos símbolos:");
        for symbol in &self.alphabet {
            print!(" {} ", symbol);
        }

        println!("");
    }
}


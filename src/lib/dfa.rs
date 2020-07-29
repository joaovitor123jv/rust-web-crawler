use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Transition {
    from: String,
    to: String,
    when: char
}

// Deterministic Finite Automaton
pub struct DFA {
    initial_state: String,
    final_state: String,
    alphabet: HashSet<char>,
    states: HashSet<String>, // Conjunto de Estados
    transitions: HashSet<Transition> // Função de transição
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
            states: obtained_states,
            transitions: HashSet::new()
        })
    }

    pub fn add_transition(&mut self, from_state: &str, to_state: &str, symbol: char) -> Result<bool, String> {
        if !self.states.contains(from_state) { 
            return Err("O estado de origem precisa pertencer ao conjunto de estados do automato".to_string());
        }

        if !self.states.contains(to_state) { 
            return Err("O estado de destino precisa pertencer ao conjunto de estados do automato".to_string());
        }

        if !self.alphabet.contains(&symbol) { 
            return Err("O simbolo recebido precisa pertencer ao alfabeto do automato".to_string());
        }

        self.transitions.insert(Transition {
            from: from_state.to_string(),
            to: to_state.to_string(),
            when: symbol
        });

        Ok(true)
    }

    pub fn test(&self, input_chain: &str) -> bool {
        let mut actual_state: String = self.initial_state.clone();

        for input in input_chain.chars() {
            if !self.alphabet.contains(&input) {
                return false;
            }

            let next_transition: Vec<&Transition> = self.transitions.iter().filter(|&t| (t.when == input) && (t.from == actual_state)).collect();

            if next_transition.first() == None {
                return false;
            }

            actual_state = next_transition.first().unwrap().to.clone();
        }

        if actual_state == self.final_state {
            true
        } else {
            false
        }
    }

    pub fn show(&self) {
        println!("-----------------");
        println!("O estado inicial é: {}, O estado final é: {}", self.initial_state, self.final_state);

        println!("\nOs estados do DFA são:");

        for state in &self.states {
            print!(" {} ", state);
        }


        println!("\nO alfabeto é composto pelos símbolos:");
        for symbol in &self.alphabet {
            print!(" {} ", symbol);
        }

        println!("\nAs transições disponíveis são:");

        for transition in &self.transitions {
            println!(" {}({}): {} ", transition.from, transition.when, transition.to);
        }

        println!("-----------------");
    }
}


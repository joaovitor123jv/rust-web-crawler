use std::collections::{HashSet, HashMap};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Transition {
    when: char,
    to: String
}

// Deterministic Finite Automaton
pub struct DFA {
    pub initial_state: String,
    pub final_state: String,
    pub actual_state: String,
    alphabet: HashSet<char>,
    states: HashSet<String>,
    transitions: HashMap<String, Vec<Transition>>
}

impl DFA {
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
            actual_state: initial_state.to_string(),
            alphabet: obtained_alphabet,
            states: obtained_states,
            transitions: HashMap::new()
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

        let raw_transitions = self.transitions.get(from_state);
        let mut raw_transitions: Vec<Transition> = match raw_transitions {
            Some(transition) => transition.clone(),
            None => Vec::new()
        };

        let requested_transition = Transition { when: symbol, to: to_state.to_string() };

        if raw_transitions.contains(&requested_transition) {
            Ok(true)
        } else {
            raw_transitions.push(Transition {
                when: symbol,
                to: to_state.to_string()
            });

            self.transitions.insert(from_state.to_string(), raw_transitions);
            Ok(true)
        }
    }

    // pub fn register(&mut self, input: char) -> bool {
    //     if !self.alphabet.contains(&input) {
    //         return false;
    //     }
    //
    //     let state_transitions = self.transitions.get(&self.actual_state);
    //
    //     let state_transitions: Vec<Transition> = match state_transitions {
    //         Some(transition) => transition.clone(),
    //         None => Vec::new()
    //     };
    //
    //     let next_transition: Vec<&Transition> = state_transitions
    //         .iter()
    //         .filter(|&transition| transition.when == input)
    //         .collect();
    //
    //     if next_transition.first() == None {
    //         false
    //     } else {
    //         self.actual_state = next_transition.first().unwrap().to.clone();
    //         true
    //     }
    // }

    pub fn test(&self, input_chain: &str) -> bool {
        let mut actual_state: String = self.initial_state.clone();

        for input in input_chain.chars() {
            if !self.alphabet.contains(&input) {
                return false;
            }

            let state_transitions = self.transitions.get(&actual_state);
            let state_transitions: Vec<Transition> = match state_transitions {
                Some(transition) => transition.clone(),
                None => Vec::new()
            };

            let next_transition: Vec<&Transition> = state_transitions
                .iter()
                .filter(|&transition| transition.when == input)
                .collect();

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

    // pub fn show(&self) {
    //     println!("-----------------");
    //     println!("O estado inicial é: {}, O estado final é: {}", self.initial_state, self.final_state);
    //
    //     println!("\nOs estados do DFA são:");
    //
    //     for state in &self.states {
    //         print!(" {} ", state);
    //     }
    //
    //
    //     println!("\nO alfabeto é composto pelos símbolos:");
    //     for symbol in &self.alphabet {
    //         print!(" {} ", symbol);
    //     }
    //
    //     println!("\nAs transições disponíveis são:");
    //
    //     for transition in &self.transitions {
    //         println!("{:?}", transition);
    //     }
    //
    //     println!("-----------------");
    // }
}


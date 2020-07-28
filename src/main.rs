mod lib;

fn test_chain(chain: &str, dfa: &lib::DFA) {
    println!("O resultado do teste da cadeia {} foi {}", chain, dfa.test(chain).unwrap());
}

fn add_generic_transition(origin_state: &str, destiny_state: &str, dfa: &mut lib::DFA) {
    dfa.add_transition(origin_state, destiny_state, '0').unwrap();
    dfa.add_transition(origin_state, destiny_state, '1').unwrap();
    dfa.add_transition(origin_state, destiny_state, '2').unwrap();
    dfa.add_transition(origin_state, destiny_state, '3').unwrap();
    dfa.add_transition(origin_state, destiny_state, '4').unwrap();
    dfa.add_transition(origin_state, destiny_state, '5').unwrap();
    dfa.add_transition(origin_state, destiny_state, '6').unwrap();
    dfa.add_transition(origin_state, destiny_state, '7').unwrap();
    dfa.add_transition(origin_state, destiny_state, '8').unwrap();
    dfa.add_transition(origin_state, destiny_state, '9').unwrap();


    dfa.add_transition(origin_state, destiny_state, 'z').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'y').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'x').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'w').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'v').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'u').unwrap();
    dfa.add_transition(origin_state, destiny_state, 't').unwrap();
    dfa.add_transition(origin_state, destiny_state, 's').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'r').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'q').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'p').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'o').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'n').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'm').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'l').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'k').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'j').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'i').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'h').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'g').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'f').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'e').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'd').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'c').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'b').unwrap();
    dfa.add_transition(origin_state, destiny_state, 'a').unwrap();
}

fn main() {
    println!("Testando importação de funcoes");

    let mut dfa = lib::DFA::new(
        "user", 
        "user_type", 
        "0123456789abcdefghijklmnopqrstuvwxyz@.", 
        &["user", "at", "provider", "user_type", "trash"]
    ).unwrap();

    add_generic_transition("trash", "trash", &mut dfa);
    dfa.add_transition("trash", "trash", '@').unwrap();
    dfa.add_transition("trash", "trash", '.').unwrap();


    add_generic_transition("at", "provider", &mut dfa);
    dfa.add_transition("at", "trash", '@').unwrap();
    dfa.add_transition("at", "trash", '.').unwrap();

    add_generic_transition("provider", "provider", &mut dfa);
    dfa.add_transition("provider", "user_type", '.').unwrap();
    dfa.add_transition("provider", "trash", '@').unwrap();

    add_generic_transition("user_type", "user_type", &mut dfa);
    dfa.add_transition("user_type", "trash", '@').unwrap();
    dfa.add_transition("user_type", "trash", '.').unwrap();

    add_generic_transition("user", "user", &mut dfa);
    dfa.add_transition("user", "at", '@').unwrap();


    dfa.show();

    // Aceita
    // Rejeita
    
    test_chain("akjdfhiowb", &dfa);
    test_chain("akjdfhiowb@asdlk.com", &dfa);
    test_chain("akjdfhiowb@asdlkcom", &dfa);
    test_chain("@asdlkcom.asd", &dfa);

    test_chain("sgreyluy@twitch.com", &dfa);
    
}


use crate::lib;

// TODO: Diferenciar emails com caracteres maiúsculos e minusculos
fn add_generic_transition(origin_state: &str, destiny_state: &str, dfa: &mut lib::DFA) -> Result<bool, String> {
    dfa.add_transition(origin_state, destiny_state, '0')?;
    dfa.add_transition(origin_state, destiny_state, '1')?;
    dfa.add_transition(origin_state, destiny_state, '2')?;
    dfa.add_transition(origin_state, destiny_state, '3')?;
    dfa.add_transition(origin_state, destiny_state, '4')?;
    dfa.add_transition(origin_state, destiny_state, '5')?;
    dfa.add_transition(origin_state, destiny_state, '6')?;
    dfa.add_transition(origin_state, destiny_state, '7')?;
    dfa.add_transition(origin_state, destiny_state, '8')?;
    dfa.add_transition(origin_state, destiny_state, '9')?;

    dfa.add_transition(origin_state, destiny_state, 'z')?;
    dfa.add_transition(origin_state, destiny_state, 'y')?;
    dfa.add_transition(origin_state, destiny_state, 'x')?;
    dfa.add_transition(origin_state, destiny_state, 'w')?;
    dfa.add_transition(origin_state, destiny_state, 'v')?;
    dfa.add_transition(origin_state, destiny_state, 'u')?;
    dfa.add_transition(origin_state, destiny_state, 't')?;
    dfa.add_transition(origin_state, destiny_state, 's')?;
    dfa.add_transition(origin_state, destiny_state, 'r')?;
    dfa.add_transition(origin_state, destiny_state, 'q')?;
    dfa.add_transition(origin_state, destiny_state, 'p')?;
    dfa.add_transition(origin_state, destiny_state, 'o')?;
    dfa.add_transition(origin_state, destiny_state, 'n')?;
    dfa.add_transition(origin_state, destiny_state, 'm')?;
    dfa.add_transition(origin_state, destiny_state, 'l')?;
    dfa.add_transition(origin_state, destiny_state, 'k')?;
    dfa.add_transition(origin_state, destiny_state, 'j')?;
    dfa.add_transition(origin_state, destiny_state, 'i')?;
    dfa.add_transition(origin_state, destiny_state, 'h')?;
    dfa.add_transition(origin_state, destiny_state, 'g')?;
    dfa.add_transition(origin_state, destiny_state, 'f')?;
    dfa.add_transition(origin_state, destiny_state, 'e')?;
    dfa.add_transition(origin_state, destiny_state, 'd')?;
    dfa.add_transition(origin_state, destiny_state, 'c')?;
    dfa.add_transition(origin_state, destiny_state, 'b')?;
    dfa.add_transition(origin_state, destiny_state, 'a')?;

    dfa.add_transition(origin_state, destiny_state, 'Z')?;
    dfa.add_transition(origin_state, destiny_state, 'Y')?;
    dfa.add_transition(origin_state, destiny_state, 'X')?;
    dfa.add_transition(origin_state, destiny_state, 'W')?;
    dfa.add_transition(origin_state, destiny_state, 'V')?;
    dfa.add_transition(origin_state, destiny_state, 'U')?;
    dfa.add_transition(origin_state, destiny_state, 'T')?;
    dfa.add_transition(origin_state, destiny_state, 'S')?;
    dfa.add_transition(origin_state, destiny_state, 'R')?;
    dfa.add_transition(origin_state, destiny_state, 'Q')?;
    dfa.add_transition(origin_state, destiny_state, 'P')?;
    dfa.add_transition(origin_state, destiny_state, 'O')?;
    dfa.add_transition(origin_state, destiny_state, 'N')?;
    dfa.add_transition(origin_state, destiny_state, 'M')?;
    dfa.add_transition(origin_state, destiny_state, 'L')?;
    dfa.add_transition(origin_state, destiny_state, 'K')?;
    dfa.add_transition(origin_state, destiny_state, 'J')?;
    dfa.add_transition(origin_state, destiny_state, 'I')?;
    dfa.add_transition(origin_state, destiny_state, 'H')?;
    dfa.add_transition(origin_state, destiny_state, 'G')?;
    dfa.add_transition(origin_state, destiny_state, 'F')?;
    dfa.add_transition(origin_state, destiny_state, 'E')?;
    dfa.add_transition(origin_state, destiny_state, 'D')?;
    dfa.add_transition(origin_state, destiny_state, 'C')?;
    dfa.add_transition(origin_state, destiny_state, 'B')?;
    dfa.add_transition(origin_state, destiny_state, 'A')?;

    Ok(true)
}

pub fn create_email_recognizer() -> Result<lib::DFA, String> {
    let mut dfa = lib::DFA::new(
        "initial", 
        "user_type", 
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@.", 
        &["initial", "user", "at", "provider", "user_filled","user_type", "trash"]
    )?;

    add_generic_transition("initial", "user", &mut dfa)?;

    add_generic_transition("user", "user", &mut dfa)?;
    dfa.add_transition("user", "user", '.')?;
    dfa.add_transition("user", "at", '@')?;

    add_generic_transition("trash", "trash", &mut dfa)?;
    dfa.add_transition("trash", "trash", '@')?;
    dfa.add_transition("trash", "trash", '.')?;


    add_generic_transition("at", "provider", &mut dfa)?;
    dfa.add_transition("at", "trash", '@')?;
    dfa.add_transition("at", "trash", '.')?;

    add_generic_transition("provider", "provider", &mut dfa)?;
    dfa.add_transition("provider", "user_filled", '.')?;
    dfa.add_transition("provider", "trash", '@')?;

    add_generic_transition("user_filled", "user_type", &mut dfa)?;
    dfa.add_transition("user_filled", "trash", '.')?;
    dfa.add_transition("user_filled", "trash", '@')?;

    add_generic_transition("user_type", "user_type", &mut dfa)?;
    dfa.add_transition("user_type", "trash", '@')?;
    dfa.add_transition("user_type", "trash", '.')?;

    Ok(dfa)
}

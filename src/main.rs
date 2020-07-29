mod lib;

fn test_chain(chain: &str, dfa: &lib::DFA) {
    println!("O resultado do teste da cadeia {} foi {}", chain, dfa.test(chain));
}


fn get_page() -> String {
    let html_page = r###" 
        <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"> <html xmlns="http://www.w3.org/1999/xhtml" > <head> <title>Estrutura do E-mail (Endereço Eletrônico) </title> </head> <body style="font-size: 12pt"> <br> <p class="ListParagraph" style="margin: 0cm 0cm 0pt 36pt; text-indent: -18pt; line-height: 150%; text-align: center; mso-list: l0 level1 lfo1"> <b><span style="font-size: 14pt; line-height: 150%; font-family: 'Arial','sans-serif'">Estrutura do E-mail (Endereço Eletrônico) </span></b> </p> <br>            <div align=center> <IMG height=16 src="images/REGUA.gif" width=560> </div> <br><br> <TABLE border=0 align=center width=80%> <TR> <TD> <div align=justify> <font face=Arial size=3 color=#000000>	Precisamos informar nosso endereço correto e completo para recebermos correspondências via correio, não é? Devemos fazer o mesmo para receber e-mails (mensagens via internet), portanto nosso endereço eletrônico também tem que estar correto.  <br><br> Como criar um e-mail de forma correta? Observe o exemplo abaixo: <br><br> <b>cjsouza@hotmail.com</b>, onde: <br><br> <b>Jcsouza</b> – é o nome escolhido pelo usuário. Talvez seu nome seja João Carlos Souza, então foi realizado um agrupamento de letras para se chegar a essa palavra; <br><br> <b>@ (arroba)</b> – símbolo que identifica o endereço de e-mail e significa que o endereço está hospedado; <br><br> <b>Hotmail</b> – nome do provedor; <br><br> <b>.com</b> – Tipo de usuário. É necessário que haja um ponto final (.) entre o provedor e o tipo.  <br><br> Seguem os tipos mais utilizados para: <br><br> <ul> <li><b>.com:</b> fins comerciais.</li> <br><br> <li><b>.org:</b> organizações sem fins lucrativos.</li> <br><br> <li><b>.edu:</b> organizações educacionais (universidades, escolas etc.).</li> <br><br> <li><b>.gov:</b> governamental.</li> </ul> Embora não tenha feito parte do exemplo anterior, em alguns casos você poderá encontrar e-mails com a sigla do país do provedor (.br – Brasil, .pt – Portugal, .uk – Inglaterra). Por exemplo,<b> ...@yahoo.com.br.</b> <br><br> Os endereços na Internet levam em consideração letras maiúsculas e minúsculas. Por exemplo: <br><br> <b>jcsouza@hotmail.com</b> é diferente de<b> JCSOUZA@hotmail.com</b>, que por sua vez é diferente de <b>JCsouza@hotmail.com</b>.  </font> </div>	</TD> </TR> </TABLE> <br><br> <div align=center> <IMG height=16 src="images/REGUA.gif" width=560> </div> <br><br> <TABLE border=0 align=center> <TR> <TD> <A href="Enviando e recebendo E-mails (mensagens).htm"><IMG height=30 alt="Pressione ENTER para retornar a página anterior." src="images/RET.gif" width=30 border=0></A> <IMG height=17 src="images/branco.gif" width=85> <A href="index.htm"><IMG height=39 alt="Pressione ENTER para retornar ao Menu. " src="images/menu.gif" width=75 border=0></A><IMG height=17 src="images/branco.gif" width=85> </TD> </TR> </TABLE> </body> </html>
    "###;

    html_page.to_string()
}



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

    Ok(true)
}

fn create_email_recognizer() -> Result<lib::DFA, String> {
    let mut dfa = lib::DFA::new(
        "initial", 
        "user_type", 
        "0123456789abcdefghijklmnopqrstuvwxyz@.", 
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

fn main() {
    println!("Testando importação de funcoes");

    let dfa = create_email_recognizer().unwrap();

    // dfa.show();

    let html_page: String = get_page();

    let mut contains_lt = false;
    let mut contains_gt = false;

    for token in html_page.split_whitespace() {
        println!("Encontrado token: {}", token);

        if token.contains("<") { contains_lt = true; }
        if token.contains(">") { contains_gt = true; } 

        if contains_gt && contains_lt {
            // Uma tag HTML ou XML
            // TODO: Implementar split_html_tags
            let html_tokens = split_html_tags(token);

            "<ashjdgasjkd>"
            "<ashjdgasjkd/>"
            "</ashjdgasjkd>"



        } else if !(contains_lt || contains_gt) && dfa.test(token) {
            println!("Email encontrado: {}", token);
        } 
    }

    // test_chain("akjdfhiowb", &dfa);
    // test_chain("akjdfhiowb@asdlk.com", &dfa);
    // test_chain("akjdfhiowb@asdlkcom", &dfa);
    // test_chain("@asdlkcom.asd", &dfa);
    // test_chain("asd@asdlkcom.", &dfa);
    // test_chain("asd@.asd", &dfa);
    //
    // test_chain("sgreyluy@twitch.com", &dfa);
}


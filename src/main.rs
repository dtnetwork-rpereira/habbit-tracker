static mut V_TEST: u8 = 255;

// TODO validar conceito de onwership e anotar no obsidian

fn main() {
    unsafe {
        V_TEST = 240;
        println!("Number: {}", V_TEST);
    }

    let mut first_number: i8 = 20;
    let mut second_number: i8 = 50;

    std::mem::swap(&mut first_number, &mut second_number);

    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);

    tabuada(5);
    // let mut nome = String::new();

    // println!("Digite seu nome: ");
    // std::io::stdin().read_line(&mut nome);
    // println!("Seu nome é: \"{}\"", nome.trim());

    asterisk(3);
    domatch();

    println!(
        "{}",
        match error_handler(true) {
            Ok(s) => s,
            Err(e) => e,
        }
    )
}

fn tabuada(n: u8) {
    let mut counter: u8 = 0;
    // panicatthedisco();
    loop {
        counter += 1;

        if counter % 2 == 1 {
            continue;
        }

        println!("{} x {} = {}", n, counter, n * counter);

        if counter == 10 {
            break;
        }
    }
}

fn asterisk(n: u8) {
    for counter in 1..=n {
        println!("{}", counter);
    }
}

fn domatch() {
    // let lang = "pt_BR";

    // let country = match lang {
    //     "pt_BR" => "Brasil",
    //     "en_US" => "Estados Unidos",
    //     _ => "Angola",
    // };

    // println!("Country: {}", country);

    for counter in 1..=10 {
        println!(
            "{}: {}",
            counter,
            match counter {
                // _ if counter % 2 == 1 => "Ímpar",
                _ if counter % 2 == 0 => "Par",
                3 => "awea",
                _ => "?",
            }
        );
    }
}

// fn panicatthedisco() {
//     panic!("aaaaaaaaaaaaaaaAAAAAAAAAAAAAAAAAAAAAAA");
// }

fn error_handler(suc: bool) -> Result<String, String> {
    if suc {
        Ok(String::from("Success"))
    } else {
        Err(String::from("error.2"))
    }
}

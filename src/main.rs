static mut V_TEST: u8 = 255;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

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
    );

    enums();
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

fn enums() {
    enum TestingLibrary {
        jest,
        cypress,
        vitest,
        other { id: u8, id2: u8 },
    }
    let selected_library: TestingLibrary = TestingLibrary::other { id: 1, id2: 1 };

    println!(
        "A biblioteca de testes selecionada foi: {}",
        match selected_library {
            TestingLibrary::jest => "Jest",
            TestingLibrary::cypress => "Cypress",
            TestingLibrary::vitest => "VI Test",
            TestingLibrary::other { id: 1, id2: 1 } => "Received all ids",
            TestingLibrary::other { id: _, id2: 1 } => "Received ID 2",
            TestingLibrary::other { id: 1, id2: _ } => "Received ID 1",
            TestingLibrary::other { id: _, id2: _ } => "Don't received nothing",
        }
    );
}

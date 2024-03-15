fn main() {
    let conta: Conta = Conta {
        titular: Titular {
            nome: String::from("Ryan"),
            sobrenome: String::from("Pereira"),
        },
        saldo: 2.5,
    };

    println!(
        "Titular: {} {}. Saldo: {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}

struct Titular {
    nome: String,
    sobrenome: String,
}
struct Conta {
    titular: Titular,
    saldo: f32,
}

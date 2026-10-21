// Genera un número aleatorio y permite que el usuario intente adivinarlo, indicando si debe probar con uno mayor o menor.
use std::io::stdin;
use rand::RngExt;

fn input_number() -> i32 {
    let mut input = String::new();
    println!("Adivina el numero entre 1 y 100");
    let _ = stdin().read_line(&mut input);
    let result = input.trim().parse().expect("numero invalido");
    return result;
}

fn main() {

    // Random number
    let mut rng = rand::rng();
    let random_number: i32 = rng.random_range(1..=100);

    let mut num = input_number();

    while num != random_number {
        if num < random_number {
            println!("Es mayor");
        } else if num > random_number {
            println!("Es menor");
        }

        num = input_number();
    }

    println!("Adivinaste el numero: {}", random_number);
}

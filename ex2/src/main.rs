// * Solicita dos números y muestra su suma, resta, multiplicación y división.

use std::io;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    println!("digita el primer numero");
    let _ = io::stdin().read_line(&mut line1);

    println!("digite el segundo numero");
    let _ = io::stdin().read_line(&mut line2);

    let num1: f32 = line1.trim_end().parse().expect("no es un numero valido");
    let num2: f32 = line2.trim_end().parse().expect("no es un numero valido");

    println!("La suma es {} ", num1 + num2);
    println!("La resta es {} ", num1 - num2);
    println!("La multiplicacion es {} ", num1 * num2);
    println!("La division es {} ", num1 / num2);
}
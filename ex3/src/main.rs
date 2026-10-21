// * Recibe un entero e indica si es par o impar. También indica si es positivo, negativo o cero.

use std::io;

fn main() {
    let mut line1 = String::new();

    println!("Digita el numero");
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut line1);
    
    let num1: i32 = line1.trim().parse().expect("numero invalido");

    if num1 % 2 == 0 {
        println!("Es par");
    } else {
        println!("Es impar");
    }
    
    if num1 > 0 {
        println!("Es positivo");
    } else if num1 == 0 {
        println!("Es cero");
    } else {
        println!("Es negativo");
    }
}

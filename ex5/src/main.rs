// * Muestra la tabla de multiplicar de un número entre el 1 y el 10.
use std::io::stdin;

fn main() {

    let mut input = String::new();

    println!("Digite el numero de la tabla de multiplicar");
    let _ = stdin().read_line(&mut input);

    let num: i32 = input.trim().parse().expect("Numero invalido");

    println!("Tabla de multiplicar");
    for i in 1..11 {
        println!("{} x {} = {}", num, i, num * i);
    }
}

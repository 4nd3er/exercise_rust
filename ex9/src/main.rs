// * Comprueba si una palabra se lee igual de izquierda a derecha que de derecha a izquierda.
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut pal_letras = Vec::new();

    println!("Digita la palabra");
    let _ = stdin().read_line(&mut input);

    for letter in input.chars() {
        pal_letras.insert(0, letter);
    }

    let palindromo: String = pal_letras.iter().collect();

    if input.trim() == palindromo.trim() {
        println!("Es un palindromo");
    } else {
        println!("No es un palindromo");
    }
}

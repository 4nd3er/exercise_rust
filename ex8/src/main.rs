// * Recibe una frase y cuenta cuántas vocales contiene, sin distinguir mayúsculas de minúsculas.
use std::io::stdin;

fn main() {
    let list_letter = vec!['a', 'e', 'i', 'o', 'u'];
    let mut input = String::new();

    println!("Digite la frase");
    let _ = stdin().read_line(&mut input);

    let mut counter = 0;
    for letter in input.to_lowercase().chars() {
        let match_letter = list_letter.contains(&letter);
        if match_letter { counter += 1 };
    }

    /*
    * Codigo Ia:
     let counter = input
        .to_lowercase()
        .chars()
        .filter(|letter| vocales.contains(letter))
        .count();
    */

    let plural = if counter != 1 { "es" } else { "" };

    println!("La frase tiene {} vocal{}", counter, plural);
}

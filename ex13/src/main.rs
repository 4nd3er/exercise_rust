// * Escribe una función que reciba una referencia mutable a un vector y agregue un elemento.
use std::io::stdin;

fn add_vec (vector: &mut Vec<i32>, num: i32) {
    vector.push(num);
} 

fn main() {
    let mut input = String::new();
    let mut vector = vec![1, 5, 20, 23, 25];

    println!("{:#^40}", "vectores mutables".to_string());
    println!("");
    println!("Digita un numero");

    let _ = stdin().read_line(&mut input);

    let num: i32 = input.trim().parse().expect("Numero invalido");
    add_vec(&mut vector, num);

    println!("{:?}", vector);
}

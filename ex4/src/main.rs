// Convierte grados Celsius a Fahrenheit y viceversa.
use std::io;

fn c_to_f (grade: f32) -> f32 {
    (grade * 1.8) + 32.0
}

fn f_to_c (grade: f32) -> f32 {
    (grade - 32.0) / 1.8
}

fn main() {

    let mut input = String::new();

    println!("Digite los grados");
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);

    let grade: f32 = input.replace(",", ".").trim().parse().expect("Numero invalido");

    let result_c_f = c_to_f(grade);
    let result_f_c = f_to_c(grade);

    println!("Celsius a Farenheit es: {}", result_c_f.to_string());
    println!("Farenheit a Celsius es: {}", result_f_c.to_string());
}

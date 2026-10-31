// * Crea un programa que solicite el año de nacimiento y calcule la edad aproximada de la persona en 2026.
use std::{io::stdin, time::SystemTime};

fn calculator(year: u64) -> Result<i32, String> {
    let result ;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let current_time = n.as_secs() / 60 / 60 / 24 / 365 + 1970;
            if year > current_time {
                return Err("El año es mayor al actual".to_string());
            }
            result = current_time - (year);
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    Ok(result as i32)
}

fn main() {
    let mut input = String::new();

    println!("Digita tu año de nacimiento");
    let _ = stdin().read_line(&mut input);

    let num_input: u64 = input.trim().parse().unwrap_or_default();
    match calculator(num_input) {
        Ok(result) => {
            let age_output = if result == 1 {
                "año"
            } else {
                "años"
            };
            println!("Tu edad actual es: {} {}", result, age_output);
        },
        Err(error) => println!("{}", error)
    };
}

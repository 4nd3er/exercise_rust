// * Crea una función que reciba una referencia a un String y cuente sus caracteres sin tomar posesión de la cadena.

fn count_charac (string: &String) -> usize {
    // string.len() // ! ❌ Asi esta mal ya que len cuenta los bytes
    string.chars().count() // * ✅ De esta manera cuenta bien los caracteres
}

fn main() {
    let string = String::from("Hello, World!");

    let total = count_charac(&string);

    println!("el total de caracteres es: {}", total.to_string());
    println!("la palabra es: {}", string);
}

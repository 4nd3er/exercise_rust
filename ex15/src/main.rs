// * Crea una calculadora que devuelva un error cuando se intente dividir entre cero, en lugar de finalizar el programa inesperadamente.

// fn calc (a: i32, b: i32) -> Option<i32> {
//     if b == 0 {
//         println!("No se puede dividir entre cero");
//     } else {
//         Some(a / b)
//     }
// }

// fn main() {
//     let num1 = 23;
//     let num2 = 0;

//     let result = calc(num1, num2);

//     println!("La division es: {:?}", result);
// }

// ? Codigo IA
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Error: no se puede dividir entre cero".to_string());
    }

    Ok(a / b)
}

fn main() {
    let resultado = dividir(10.0, 0.0);

    match resultado {
        Ok(valor) => println!("Resultado: {}", valor),
        Err(error) => println!("{}", error),
    }
}
// ? Codigo IA

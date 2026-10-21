// * Crea un String, asígnalo a otra variable e intenta utilizar ambas. Después soluciona el problema utilizando clone() y explica la diferencia.

fn main() {
    let s1 = String::from("hola");
    // let s2 = s1; // s1 se "mueve" a s2. s2 es ahora el dueño.
    // println!("{}", s1); // Error de compilación: s1 ya no es válida
    
    let s2 = s1.clone(); // s1 se clona y la pertenencia no se mueve
    println!("{}", s1); // Ya no daria error con el clone
    println!("{}", s2);   // Funciona bien

}

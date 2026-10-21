// Guarda varios números en un vector e identifica el mayor, el menor, el promedio y la suma.

fn main() {
    let mut vector = Vec::new();
    for i in 1..=10 {
        vector.push(i);
    }

    let mut max = vector[0];
    let mut min = vector[0];
    let mean: f32;
    let mut sum = 0;

    for &i in &vector {
        if i > max {
            max = i;
        } else if i < min {
            min = i;
        }
        sum += i;
    }

    let length_v: i32 = vector.len().try_into().unwrap();
    mean = sum as f32 / length_v as f32;

    println!("El numero maximo es: {}", max);
    println!("El numero minimo es: {}", min);
    println!("El promedio es: {}", mean);
    println!("La suma total es: {}", sum);
}


// ? Opción con IA
/*
fn main() {
    // Creamos el vector directamente usando un rango y collect
    let vector: Vec<i32> = (1..=10).collect();

    // 1. Calculamos la suma (consumiendo el iterador con .sum())
    let sum: i32 = vector.iter().sum();

    // 2. Obtenemos el máximo y mínimo
    // .max() y .min() devuelven un Option (por si el vector está vacío), 
    // y devuelven una referencia, así que usamos * para obtener el número real.
    let max = *vector.iter().max().unwrap_or(&1);
    let min = *vector.iter().min().unwrap_or(&1);

    // 3. Calculamos el promedio
    let length_v = vector.len() as i32;
    let mean = sum / length_v;

    println!("El numero maximo es: {}", max);
    println!("El numero minimo es: {}", min);
    println!("El promedio es: {}", mean);
    println!("La suma total es: {}", sum);
}

*
¿Qué cambió aquí?
*
• (1..=10).collect(): En lugar de hacer un bucle for para meter los números con .push(), Rust puede transformar un rango directamente en un Vec usando .collect().
*
• .unwrap_or(&1): Como el vector podría estar vacío, los métodos .max() y .min() devuelven un tipo seguro llamado Option. Usamos unwrap_or para decirle: "Dame el valor máximo, y si el vector llega a estar vacío, por defecto toma un 1".
*
• as i32: Es la forma más rápida y directa en Rust de transformar un tipo numérico a otro (el tamaño .len() que es usize lo convertimos a i32).
*/

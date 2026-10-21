// * Implementa una función que busque un número en un vector y devuelva Some(posición) o None.

fn search_num (vector: &Vec<i32>, num: i32) -> Option<&i32> {
    let result: Vec<_> = vector.iter().filter(| x | **x == num).collect();
    Some(result[0])
}

fn main() {
    let vector = vec![1, 23, 45, 67, 90];
    let num_to_search = 23;

    let result = search_num(&vector, num_to_search);

    println!("{:?}", result);
}

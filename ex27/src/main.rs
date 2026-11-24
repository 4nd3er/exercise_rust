// * Crea un iterador personalizado que genere números de Fibonacci.
// ? EXTRA: Crear un iterador para los numeros primos

struct NumFib {
    vec: Vec<i128>,
    temp: i128,
    a: i128,
    b: i128
}

impl NumFib {
    fn iterator () -> NumFib {
        NumFib { vec: Vec::new(), temp: 1, a: 0, b: 1 }
    }
}

impl Iterator for NumFib {
    type Item = i128;
    fn next(&mut self) -> Option<Self::Item> {
        self.temp = self.a + self.b;
        self.a = self.b;
        self.b = self.temp;
        let result = Some(self.b);
        self.vec.push(self.b);
        result
    }
}


struct PrimerNumber {
    vec: Vec<i128>,
    num: i128
}

impl PrimerNumber {
    fn iterator () -> PrimerNumber {
        PrimerNumber { vec: Vec::new(), num: 2 }
    }
}

impl Iterator for PrimerNumber {
    type Item = i128;
    fn next(&mut self) -> Option<Self::Item> {
        let mut temp: i128 = self.num;
        loop {
            let result = self.vec.iter().find(|x| temp % *x == 0);
            match result {
                Some(_) => {
                    temp += 1;
                }
                None => {
                    self.vec.push(temp);
                    break;
                }
            }
        }
        self.num = temp;
        Some(temp)
    }
}

fn main() {
    let mut vec_num_fib = NumFib::iterator();
    let mut vec_prime_numbers = PrimerNumber::iterator();

    for _ in 0..5 {
        println!("{:?}", vec_num_fib.next());
    }

    println!("");
    for _ in 0..5 {
        println!("{:?}", vec_prime_numbers.next());
    }
    

    // ? uncomment for test
    // for _ in 0..100 {
    //     println!("{:?}", vec_num_fib.next());
    // }
}

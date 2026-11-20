// * Crea un iterador personalizado que genere números de Fibonacci.

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

fn main() {
    let mut vec_num_fib: NumFib = NumFib::iterator();

    for _ in 0..5 {
        println!("{:?}", vec_num_fib.next());
    }


    // ? uncomment for test
    // for _ in 0..100 {
    //     println!("{:?}", vec_num_fib.next());
    // }
}

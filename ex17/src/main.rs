// * Registra productos con nombre, precio y cantidad. Calcula el valor total del inventario.
use std::io::stdin;

struct Product {
    name: String,
    price: f32,
    quantity: i32,
}

fn p(string: &str) {
    println!("{string}");
}

fn pr_a_std(string: &str) -> String {
    let mut input = String::new();
    p(string);
    let _ = stdin().read_line(&mut input);
    input
}

fn add_product(products: &mut Vec<Product>, product: Product) {
    products.push(product);
}

fn main() {
    let mut products: Vec<Product> = Vec::new();

    loop {
        p("add product (1)");
        p("calculate the total value of inventary (2)");
        p("exit the program (0)");
        let instruction = pr_a_std("");

        match instruction.trim() {
            "0" => break,
            "1" => {
                let name = pr_a_std("Type the name's product");
                let price = pr_a_std("Type the price's product");
                let quantity = pr_a_std("Type the quantity's product");

                let new_product = Product {
                    name: name.trim().to_string(),
                    price: price.trim().parse().unwrap_or(0.0),
                    quantity: quantity.trim().parse().unwrap_or(0)
                };

                let name_product = new_product.name.clone();
                add_product(&mut products, new_product);
                println!("The product {name_product} is registered successfully!");
            },
            "2" => {
                let mut total_inventary : f32 = 0.0;
                for product in &products {
                    let total_product = product.price * product.quantity as f32;
                    total_inventary += total_product;
                }
                println!("The total value of inventary is ${total_inventary}");
            }
            _ => println!("Invalid instruction, try again.")
        }
    }
}

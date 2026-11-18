// * Desarrolla un inventario que pueda almacenar diferentes tipos de productos.
use std::{collections::HashMap, io::stdin};

fn p_a_i(string: &str) -> String {
    println!("{string}");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    input
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Structs {
    Tech,
    Health,
    Clothes,
}

const PARAMS: [&str; 3] = ["Nombre", "Cantidad", "Precio"];
const LISTSTRUCTS: [Structs; 3] = [Structs::Tech, Structs::Health, Structs::Clothes];

trait Info {
    fn show_info(&self);
}

struct Tech {
    name: String,
    type_p: String,
    quantity: i32,
    price: i64,
}

struct Health {
    name: String,
    type_p: String,
    quantity: i32,
    price: i64,
}

struct Clothes {
    name: String,
    type_p: String,
    quantity: i32,
    price: i64,
}

impl Info for Tech {
    fn show_info(&self) {
        println!("nombre: {}", self.name);
        println!("cantidad: {}", self.quantity);
        println!("valor unitario: {}", self.price);
        println!("valor total: {}", self.price * self.quantity as i64);
    }
}

impl Info for Health {
    fn show_info(&self) {
        println!("nombre: {}", self.name);
        println!("cantidad: {}", self.quantity);
        println!("valor unitario: {}", self.price);
        println!("valor total: {}", self.price * self.quantity as i64);
    }
}

impl Info for Clothes {
    fn show_info(&self) {
        println!("nombre: {}", self.name);
        println!("cantidad: {}", self.quantity);
        println!("valor unitario: {}", self.price);
        println!("valor total: {}", self.price * self.quantity as i64);
    }
}

impl Default for Tech {
    fn default() -> Self {
        Self {
            name: "tin".to_string(),
            type_p: "tech".to_string(),
            quantity: 0,
            price: 0,
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            name: "tin".to_string(),
            type_p: "health".to_string(),
            quantity: 0,
            price: 0,
        }
    }
}

impl Default for Clothes {
    fn default() -> Self {
        Self {
            name: "tin".to_string(),
            type_p: "clothes".to_string(),
            quantity: 0,
            price: 0,
        }
    }
}

fn create_product() -> Vec<String> {
    let mut values = Vec::new();
    for param in PARAMS {
        let result = p_a_i(format!("Digita el {}", param).as_str());
        values.push(result);
    }
    values
}

fn save_product<T: Info + Default + 'static>(
    products: &mut HashMap<Structs, Vec<Box<dyn Info>>>,
    product: T,
    key: &Structs,
) {
    match products.get_mut(key) {
        Some(list) => {
            list.push(Box::new(product));
            println!("Producto registrado!");
        }
        None => println!("No se encontró el producto"),
    }
}

fn main() {
    let mut products: HashMap<Structs, Vec<Box<dyn Info>>> = HashMap::new();
    for struct_p in LISTSTRUCTS {
        products.insert(struct_p, vec![]);
    }

    loop {
        println!("(1) Añadir productos");
        println!("(2) Ver productos");
        println!("(0) Salir");
        let result = p_a_i("Que deseas hacer?");
        let instruction: i32 = result.trim().parse().unwrap_or_default();

        match instruction {
            0 => {
                println!("Hasta pronto!");
                break;
            }
            1 => {
                println!("(1) Salud");
                println!("(2) Tecnología");
                println!("(3) Ropa");
                println!("(0) Volver");
                let result = p_a_i("Digita el producto a añadir:");

                let instruction: i32 = result.trim().parse().unwrap_or_default();

                match instruction {
                    0 => {}
                    1 => {
                        let values = create_product();
                        let product = Health {
                            name: values[0].trim().to_string(),
                            quantity: values[1].trim().parse().unwrap_or_default(),
                            price: values[2].trim().parse().unwrap_or_default(),
                            ..Default::default()
                        };
                        save_product(&mut products, product, &Structs::Health);
                    }
                    2 => {
                        let values = create_product();
                        let product = Tech {
                            name: values[0].trim().to_string(),
                            quantity: values[1].trim().parse().unwrap_or_default(),
                            price: values[2].trim().parse().unwrap_or_default(),
                            ..Default::default()
                        };
                        save_product(&mut products, product, &Structs::Tech);
                    }
                    3 => {
                        let values = create_product();
                        let product = Clothes {
                            name: values[0].trim().to_string(),
                            quantity: values[1].trim().parse().unwrap_or_default(),
                            price: values[2].trim().parse().unwrap_or_default(),
                            ..Default::default()
                        };
                        save_product(&mut products, product, &Structs::Clothes);
                    }
                    _ => println!("Instrucción no valida"),
                }
            }
            2 => {
                for (key, lists) in &products {
                    println!("{:#^20}", "");
                    println!("Producto: {:?}", key);
                    for product in lists {
                        product.show_info();
                        println!("");
                    }
                    println!("{:#^20}", "");
                }
            }
            _ => println!("Instrucción no valida"),
        }
        println!("");
    }
}

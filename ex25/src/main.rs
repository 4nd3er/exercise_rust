// * Desarrolla un inventario que pueda almacenar diferentes tipos de productos.

// TODO: implementate a trait to show information from the production
// TODO: Maybe: show the quantity, the price, and so on.

use std::any::Any;

trait Info {
    fn show_info(&self);
}

struct Tech {
    name: String,
    type_p: String,

}

struct Health {
    name: String
}

impl Info for Tech {
    fn show_info(&self) {
        println!("{}", &self.type_p);
    }
}

impl Default for Tech {
    fn default() -> Self {
        Tech {
            name: "tin".to_string(),
            type_p: "tech".to_string()
        }
    }
}

impl Info for Health {
    fn show_info(&self) {
        println!("{:?}", &self.name);
    }
}

fn add_product<T: Info>(product: T, list_products: &mut Vec<T>) {
    list_products.push(product);
}

fn main() {
    let product = Tech {
        name: "tan".to_string(),
        ..Default::default()
    };
    let mut list_products = Vec::new();
    product.show_info();
    add_product(product, &mut list_products);
}

use super::product::Product;

pub struct Inventory {
    pub products: Vec<Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { products: Vec::new() }
    }

    pub fn add_product(&mut self, product: Product) {
    }

    pub fn edit_product(&mut self, product_name: &str, new_product: Product) {

    }

    pub fn delete_product(&mut self, product_name: &str) {
    }
}

use jsonway::{self, ObjectSerializer};
use time;

use super::super::model::product;

pub struct ProductSerializer;
impl jsonway::ObjectSerializer<product::Product> for ProductSerializer {
    fn root(&self) -> Option<&str> { Some("product") }
    fn build(&self, product: &product::Product, json: &mut jsonway::ObjectBuilder) {
        json.set("id", product.get_id().to_string());
        json.set("description", product.get_description().to_string());
        json.set("price", product.get_price().to_string());
        json.set("created_at", time::at_utc(tweet.get_created_at().clone()).rfc3339().to_string());
        json.set("updated_at", time::at_utc(tweet.get_updated_at().clone()).rfc3339().to_string());
    }
}

pub struct ProductListSerializer<'a> {
    products: &'a Vec<product::Product>
}

impl<'a> ProductListSerializer<'a> {
    pub fn new(products: &'a Vec<product::Product>) -> ProductListSerializer<'a> {
        ProductListSerializer {
            products,
        }
    }
}

impl<'a> jsonway::ArraySerializer for ProductListSerializer<'a> {
    fn root(&self) -> Option<&str> { Some("products") }
    fn build(&self, array: &mut jsonway::ArrayBuilder) {
        for product in self.products.iter() {
            array.push(ProductSerializer.serialize(product, false));
        }
    }
}

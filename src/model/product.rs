use time;

pub struct Product {
    id: u32,
    description: String,
    price: f32,
    created_at: time::Timespec,
    updated_at: time::Timespec
}

impl Product {

    pub fn get_id(&self) -> &u32 { &self.id }
    pub fn get_description(&self) -> &str { self.description.as_slice() }
    pub fn get_price(&self) -> &f32 { self.price }
    pub fn get_created_at(&self) -> &time::Timespec { &self.created_at }
    pub fn get_updated_at(&self) -> &time::Timespec { &self.updated_at }

    pub fn set_description(&mut self, description: String) { self.description = description; }
    pub fn set_price(&mut self, price: f32) { self.price = price; }

    pub fn new(description: String, price: f32) -> Product {
        Product {
            id: uuid::Uuid::new_v4(),
            description,
            price,
            created_at: time::get_time(),
            updated_at: time::get_time()
        }
    }

    pub fn find(id: uuid::Uuid) -> Option<Product> {
        NONE
    }

    pub fn create(&self) {

    }

    pub fn update(&mut self) {

    }

    pub fn delete(&self) {

    }
}
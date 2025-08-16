use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

pub trait PrintInfo {
    fn print_info(&self);
}

impl PrintInfo for Person {
    fn print_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("City: {}", self.city);
    }
}

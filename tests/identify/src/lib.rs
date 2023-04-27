pub use identify_derive::*;

pub trait Identifiable {
    fn id(&mut self) -> &mut String;
}

pub fn print_id<T: Identifiable>(object: &mut T) {
    println!("{}", object.id());
}

pub fn randomize_id<T: Identifiable>(object: &mut T) {
    *object.id() = rand::random::<u32>().to_string();
}
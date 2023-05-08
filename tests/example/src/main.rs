use identify::Identifiable;

#[derive(Identifiable)]
struct Foo {
    id: String,
}

fn main() {
    let mut foo = Foo {
        id: "123".to_string(),
    };
    identify::print_id(&mut foo);
    identify::randomize_id(&mut foo);
    identify::print_id(&mut foo);
}
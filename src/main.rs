mod routes;
mod domain;
mod repository;
mod use_cases;

fn main()
{
    println!("Hello world !");
    routes::serve();
}
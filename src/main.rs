mod repositories;
mod server;
pub mod contexts;

fn main()
{
    println!("Hello world !");
    server::serve();
}
mod repositories;
mod server;
pub mod contexts;

use repositories::places::repository::Repository as PlaceRepository;
use contexts::places::use_cases::UseCase as PlaceUseCase;
use server::Server;

fn main()
{
    Server::new(
        PlaceUseCase::new(
            Box::new(PlaceRepository)
        )
    ).serve();
}
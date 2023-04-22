mod repositories;
mod server;
pub mod contexts;

use server::Server;

use repositories::
{
    places::repository::Repository as PlaceRepository,
    users::repository::Repository  as UserRepository,
};

use contexts::
{
    places::use_cases::UseCase as PlaceUseCase,
    users::use_cases::UseCase  as UserUseCase,
};

fn main()
{
    Server::new(
        PlaceUseCase::new(Box::new(PlaceRepository)),
        UserUseCase::new(Box::new(UserRepository)),
    ).serve();
}
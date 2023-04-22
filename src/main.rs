mod repositories;
mod server;
pub mod contexts;

use server::Server;

use repositories::
{
    places::repository::Repository as PlaceRepository,
    routes::repository::Repository as RouteRepository,
    users::repository::Repository  as UserRepository,
};

use contexts::
{
    places::use_cases::UseCase as PlaceUseCase,
    routes::use_cases::UseCase as RouteUseCase,
    users::use_cases::UseCase  as UserUseCase,
};

fn main()
{
    Server::new(
        PlaceUseCase::new(Box::new(PlaceRepository)),
        RouteUseCase::new(Box::new(RouteRepository)),
        UserUseCase::new(Box::new(UserRepository)),
    ).serve();
}
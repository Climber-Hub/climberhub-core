mod repositories;
mod server;
pub mod contexts;
pub mod errors;

use server::Server;

use repositories::
{
    places::repository::Repository as PlaceRepository,
    routes::repository::Repository as RouteRepository,
    users::repository::Repository  as UserRepository,
};

use contexts::
{
    places::use_cases::UseCase       as PlaceUseCase,
    routes::use_cases::get::UseCase  as RouteGetUseCase,
    routes::use_cases::post::UseCase as RoutePostUseCase,
    routes::use_cases::put::UseCase  as RoutePutUseCase,
    users::use_cases::UseCase        as UserUseCase,
};

fn main()
{
    // TODO: This is not ideal to have multiple instance of the same repository, should think about sharing repo between use_cases
    Server::new(
        PlaceUseCase::new(Box::new(PlaceRepository)),
        RouteGetUseCase::new(Box::new(RouteRepository)),
        RoutePostUseCase::new(Box::new(RouteRepository)),
        RoutePutUseCase::new(Box::new(RouteRepository)),
        UserUseCase::new(Box::new(UserRepository)),
    ).serve();
}
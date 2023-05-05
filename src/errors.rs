pub enum GetAllError
{
    InternalServerError,
}

pub enum GetError
{
    NonExistingId(String),
    InternalServerError,
}
pub enum CreateError
{
    InternalServerError,
}
pub enum UpdateError
{
    NonExistingId(String),
    InternalServerError,
}
pub enum DeleteError
{
    NonExistingId(String),
    InternalServerError,
}
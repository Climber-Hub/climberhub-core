use super::domain::User;

pub trait IRepository
{
    fn get_users(&self) -> Vec<User>;
}
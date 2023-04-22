use crate::contexts::users::
{
    irepository::IRepository,
    domain::User,
};
pub struct Repository;
impl IRepository for Repository
{
    fn get_users(&self) -> Vec<User> 
    {
        vec![User {
            first_name : String::new(),
            last_name  : String::new(),
            username   : String::new(),
            email      : String::new(), 
        }]

    }
}
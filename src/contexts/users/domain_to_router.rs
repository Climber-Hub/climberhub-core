use super::{domain, router};

pub fn user(u: domain::User) -> router::User
{
    router::User 
    {
        first_name : u.first_name,
        last_name  : u.last_name,
        username   : u.username,
        email      : u.email,
    }
}

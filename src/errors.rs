pub mod get
{
    pub enum Error
    {
    }

    pub enum IdError
    {
        NonExistingId(String),
    }
}
pub mod post
{
    pub enum Error
    {
    }
}
pub mod put
{
    pub enum Error
    {
        NonExistingId(String),
    }
}
pub mod delete
{
    pub enum Error
    {
        NonExistingId(String),
    }
}
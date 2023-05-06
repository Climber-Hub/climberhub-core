#[derive(Debug)]
pub struct Place
{
    pub id   : PlaceId,
    pub data : PlaceData,
}
pub type PlaceId = String;
#[derive(Debug)]
pub struct PlaceData
{
    pub name        : String,
    pub description : String,
    pub address     : String,
    pub postcode    : String,
    pub city        : String,
    pub country     : String,
}

pub mod get
{
    #[derive(Debug)]
    pub struct Filters
    {
        pub country : Option<String>,
        pub city    : Option<String>,
    }
}
pub struct Place 
{
    pub id          : String,
    pub name        : String,
    pub description : String,
    pub address     : String,
    pub postcode    : String,
    pub city        : String,
    pub country     : String,
}

pub struct Filters
{
    pub country : Option<String>,
    pub city    : Option<String>,
}

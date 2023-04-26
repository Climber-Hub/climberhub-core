use std::collections::HashMap;

pub type RouteProperties = HashMap<String, String>;

#[derive(Debug)]
pub struct RouteData
{
    pub name        : String,
    pub description : String,
    pub grade       : String,
    pub color       : String,
    pub sector      : String,
    pub rules       : Rules,
    pub tags        : Vec<String>,
    pub properties  : RouteProperties,
}

#[derive(Debug)]
pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}

pub type RouteId = String;

#[derive(Debug)]
pub struct Route
{
    pub id   : RouteId,
    pub data : RouteData,
}

pub mod get
{
    pub struct Filters
    {
        pub min_grade  : Option<String>,
        pub max_grade  : Option<String>,
        pub tags       : Vec<String>,
        pub properties : super::RouteProperties,
    }
}

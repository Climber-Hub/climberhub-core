use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::super::places::domain::PlaceId;

pub type RouteProperties = HashMap<String, String>;
pub type RouteId = String;
pub type Date = DateTime<Utc>;

#[derive(Debug)]
pub struct RouteData
{
    pub place_id     : PlaceId,
    pub name         : String,
    pub description  : String,
    pub grade        : String,
    pub color        : String,
    pub sector       : String,
    pub opening_date : Date,
    pub closing_date : Option<Date>,
    pub rules        : Rules,
    pub tags         : Vec<String>,
    pub properties   : RouteProperties,
}

#[derive(Debug)]
pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}


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

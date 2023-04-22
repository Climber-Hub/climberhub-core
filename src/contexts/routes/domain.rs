use std::collections::HashMap;

pub struct Route
{
    pub name        : String,
    pub description : String,
    pub grade       : String,
    pub color       : String,
    pub sector      : String,
    pub rules       : Rules,
    pub tags        : Vec<String>,
    pub properties  : HashMap<String, String>,
}

pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}

pub struct Filters
{
    pub min_grade  : Option<String>,
    pub max_grade  : Option<String>,
    pub tags       : Vec<String>,
    pub properties : HashMap<String, String>,
}
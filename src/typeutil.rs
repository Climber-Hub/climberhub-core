pub mod routers
{
    use rocket::form::FromFormField;
    use rocket_okapi::{
        JsonSchema, 
        okapi::schemars::{gen::SchemaGenerator, schema::{Schema, SchemaObject, InstanceType}}
    };
    use serde::{Serialize, Deserialize};
    use chrono::{DateTime, Utc};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Date(pub DateTime<Utc>);

    impl std::ops::Deref for Date
    {
        type Target = DateTime<Utc>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl<'r> FromFormField<'r> for Date
    {
        fn from_value(field: rocket::form::ValueField<'r>) -> rocket::form::Result<'r, Self> 
        {
            match DateTime::parse_from_rfc3339(field.value)
            {
                Ok(date) => Ok(Date(date.into())),
                Err(_) => panic!(),
            }
        }
    }

    impl JsonSchema for Date
    {
        fn schema_name() -> String { String::from("Date") }
        fn json_schema(_gen: &mut SchemaGenerator) -> Schema 
        {
            SchemaObject {
                instance_type : Some(InstanceType::String.into()),
                format        : Some("date-time".to_owned()),
                ..Default::default()
            }.into()
        }
    }
}

pub mod repositories
{
    use serde::{Serialize, Deserialize};
    use chrono::{DateTime, Utc};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Date(pub DateTime<Utc>);

    impl std::ops::Deref for Date
    {
        type Target = DateTime<Utc>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
}
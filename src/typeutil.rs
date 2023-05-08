pub mod routers
{
    use rocket::form::FromFormField;
    use rocket_okapi::{
        JsonSchema, 
        okapi::schemars::{gen::SchemaGenerator, schema::{Schema, SchemaObject, InstanceType}}
    };
    use serde::{Serialize, Deserialize};
    use chrono::{NaiveDate};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Date(pub NaiveDate);

    impl std::ops::Deref for Date
    {
        type Target = NaiveDate;
        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl<'r> FromFormField<'r> for Date
    {
        fn from_value(field: rocket::form::ValueField<'r>) -> rocket::form::Result<'r, Self> 
        {
            match field.value.parse::<NaiveDate>()
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
                format        : Some("date".to_owned()),
                ..Default::default()
            }.into()
        }
    }
}

pub mod repositories
{
    use serde::{Serialize, Deserialize};
    use chrono::{NaiveDate};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Date(pub NaiveDate);

    impl std::ops::Deref for Date
    {
        type Target = NaiveDate;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
}
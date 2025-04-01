use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    pub id: i32,
    pub url: String,
    pub alt: Option<String>,
}

use crate::schema::images;

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage<'a> {
    pub url: &'a str,
    pub alt: Option<&'a str>,
}
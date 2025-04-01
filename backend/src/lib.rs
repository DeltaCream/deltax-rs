use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewImage, Image};

pub fn create_image(conn: &mut PgConnection, url: &str, alt: Option<&str>) -> Image {
    use crate::schema::images;

    let new_image_link = NewImage { url, alt };

    diesel::insert_into(images::table)
        .values(&new_image_link)
        .returning(Image::as_returning())
        .get_result(conn)
        .expect("Error saving new image link")
}
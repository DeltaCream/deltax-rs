// use serde::{Deserialize, Serialize};

// #[derive(Queryable, Serialize, Deserialize)]
// pub struct Image {
//     pub id: i32,
//     pub url: String,
//     pub alt: Option<String>,
// }

// table! {
//     images (id) {
//         id -> Int4,
//         url -> Text,
//         alt -> Nullable<Text>,
//     }
// }

// use diesel::prelude::*;
// use crate::schema::images;

// pub fn insert_image_urls(conn: &PgConnection, urls: Vec<String>) -> QueryResult<usize> {
//     let new_images: Vec<_> = urls.into_iter().map(|url| (images::url.eq(&url))).collect();
//     diesel::insert_into(images::table)
//         .values(&new_images)
//         .execute(conn)
// }

// // use crate::models::Image;

// pub fn get_all_image_urls(conn: &PgConnection) -> QueryResult<Vec<Image>> {
//     use crate::schema::images::dsl::*;
//     images.load::<Image>(conn)
// }

// CREATE TABLE images (
//     id SERIAL PRIMARY KEY,
//     url TEXT NOT NULL
// );
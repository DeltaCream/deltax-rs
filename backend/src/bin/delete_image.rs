use diesel::prelude::*;
use deltax_rs::*;
use std::env::args;

fn main() {
    // use self::schema::images::dsl::images;
    use deltax_rs::schema::images::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(images.filter(url.like(pattern)))
        .execute(connection)
        .expect("Error deleting images");

    println!("Deleted {} images", num_deleted);
}
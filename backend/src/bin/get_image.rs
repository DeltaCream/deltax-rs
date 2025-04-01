use self::models::Image;
use diesel::prelude::*;
use deltax_rs::*;
use std::env::args;

fn main() {
    // use self::schema::images::dsl::images;
    use deltax_rs::schema::images::dsl::images;

    let image_id = args()
        .nth(1)
        .expect("get_image requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let image = images
        .find(image_id)
        .select(Image::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match image {
        Ok(Some(image)) => println!("Image with id: {} has a url: {}", image.id, image.url),
        Ok(None) => println!("Unable to find image {}", image_id),
        Err(_) => println!("An error occured while fetching image {}", image_id),
    }
}
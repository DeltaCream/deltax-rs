use self::models::*;
use deltax_rs::*;
use diesel::prelude::*;

fn main() {
    use self::schema::images::dsl::*;
    let connection = &mut establish_connection();
    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .select(Post::as_select())
    //     .load(connection)
    //     .expect("Error loading posts");
    let results = images
        .limit(5)
        .select(Image::as_select())
        .load(connection)
        .expect("Error loading images");

    println!("Displaying {} image links", results.len());
    for image in results {
        println!("{}", image.url);
        println!("-----------\n");
        println!("{}", image.alt.unwrap_or("None provided".to_string()));
    }
}

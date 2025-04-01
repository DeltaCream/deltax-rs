use deltax_rs::*;
use std::io::{Read, stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut url = String::new();
    let mut alt = String::new();

    println!("What is your image url?");
    stdin().read_line(&mut url).unwrap();
    let url = url.trim_end(); // Remove the trailing newline

    println!("\nOk! Let's describe your image from {url} (Press {EOF} when finished)\n",);
    // let mut alt = String::new();
    stdin().read_to_string(&mut alt).unwrap();

    let alt = if alt.trim().is_empty() {
        None
    } else {
        Some(alt.trim().to_string())
    };

    let image = create_image(connection, url, alt.as_deref());
    println!("\nSaved image from {url} with id {}", image.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

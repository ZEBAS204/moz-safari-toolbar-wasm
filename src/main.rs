use base64::prelude::*;
use image::{load_from_memory_with_format, ImageFormat};
use lib::*;
use std::fs::File;
use std::io::{stdin, stdout, Write};

fn _main_jpg() {
    let mut input = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    const BLUR_RADIUS: i32 = 5; // Adjust the radius as needed

    // Remove the data URL prefix
    let base64_str = input.split(',').nth(1).unwrap_or("");

    println!("base64: {}", base64_str);

    // Decode
    let base64_to_vector = BASE64_STANDARD.decode(base64_str.trim()).unwrap();
    let img = match load_from_memory_with_format(&base64_to_vector, ImageFormat::Jpeg) {
        Ok(img) => img.into_rgb8(),
        Err(error) => {
            println!("ERROR! {}", error);
            panic!();
        }
    };

    let blurred_image = gaussian_blur(&img, BLUR_RADIUS);

    let result = base64_save_image(blurred_image, ImageFormat::Jpeg);

    let mut file = File::create("output.md").expect("creation failed");
    file.write_all(result.as_bytes()).expect("write failed");
    file.flush().ok(); // Flush now since its not guarantee emptying the buffer when going out of scope

    println!("Gaussian Blurring completed.");
}

fn main() {
    // Decode
    const BLUR_RADIUS: i32 = 5; // Adjust the radius as needed

    let bmp_data: &[u8] = include_bytes!("../greenland_grid_velo.bmp");
    let img = match load_from_memory_with_format(bmp_data, ImageFormat::Bmp) {
        Ok(img) => img.into_rgb8(),
        Err(error) => {
            println!("ERROR! {}", error);
            panic!();
        }
    };

    let blurred_image = gaussian_blur(&img, BLUR_RADIUS);

    // Save to file
    blurred_image.save("test.bmp").unwrap();

    let result_jpg: String = base64_save_image(blurred_image, ImageFormat::Bmp);
    let mut file = File::create("output.md").expect("creation failed");
    file.write_all(result_jpg.as_bytes()).expect("write failed");
    file.flush().ok(); // Flush now since its not guarantee emptying the buffer when going out of scope

    println!("Gaussian Blurring completed.");
}

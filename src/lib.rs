extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use base64::prelude::*;
use image::{load_from_memory_with_format, ImageBuffer, ImageFormat, Rgb};
use logging_timer::time;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

fn create_gaussian_kernel(radius: i32) -> Vec<f64> {
    let size = 2 * radius + 1;
    let sigma = radius as f64 / 2.0;
    let mut kernel = vec![0.0; size as usize];
    let mut sum = 0.0;

    for i in 0..size {
        let x = i - radius;
        kernel[i as usize] = (-((x * x) as f64) / (2.0 * sigma * sigma)).exp();
        sum += kernel[i as usize];
    }

    for i in 0..size {
        kernel[i as usize] /= sum;
    }

    kernel
}

fn apply_horizontal_blur(
    source: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    kernel: &[f64],
    radius: i32,
) {
    let width = source.width() as i32;
    let height = source.height() as i32;
    let kernel_radius = kernel.len() as i32 / 2;

    (0..height).for_each(|y| {
        for x in 0..width {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut kernel_sum = 0.0;

            for i in -radius..=radius {
                let px = x + i;
                if px >= 0 && px < width {
                    let pixel = source.get_pixel(px as u32, y as u32);
                    let kernel_value = kernel[(i + kernel_radius) as usize];

                    r += pixel[0] as f64 * kernel_value;
                    g += pixel[1] as f64 * kernel_value;
                    b += pixel[2] as f64 * kernel_value;

                    kernel_sum += kernel_value;
                }
            }

            let new_r = (r / kernel_sum) as u8;
            let new_g = (g / kernel_sum) as u8;
            let new_b = (b / kernel_sum) as u8;

            // let mut target_m = target.clone();
            target.put_pixel(x as u32, y as u32, Rgb([new_r, new_g, new_b]));
        }
    });
}

fn apply_vertical_blur(
    source: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    kernel: &[f64],
    radius: i32,
) {
    let width = source.width() as i32;
    let height = source.height() as i32;
    let kernel_radius = kernel.len() as i32 / 2;

    (0..width).for_each(|x| {
        for y in 0..height {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut kernel_sum = 0.0;

            for i in -radius..=radius {
                let py = y + i;
                if py >= 0 && py < height {
                    let pixel = source.get_pixel(x as u32, py as u32);
                    let kernel_value = kernel[(i + kernel_radius) as usize];

                    r += pixel[0] as f64 * kernel_value;
                    g += pixel[1] as f64 * kernel_value;
                    b += pixel[2] as f64 * kernel_value;

                    kernel_sum += kernel_value;
                }
            }

            let new_r = (r / kernel_sum) as u8;
            let new_g = (g / kernel_sum) as u8;
            let new_b = (b / kernel_sum) as u8;

            target.put_pixel(x as u32, y as u32, Rgb([new_r, new_g, new_b]));
        }
    });
}

#[time("info", "BLUR APPLY")]
pub fn gaussian_blur(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    radius: i32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let kernel = create_gaussian_kernel(radius);
    let mut temp_image = ImageBuffer::new(image.width(), image.height());
    let mut blurred_image = ImageBuffer::new(image.width(), image.height());

    apply_horizontal_blur(image, &mut temp_image, &kernel, radius);
    apply_vertical_blur(&temp_image, &mut blurred_image, &kernel, radius);

    blurred_image
}

#[time("info", "BASE64 encoding")]
pub fn base64_save_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>, img_format: ImageFormat) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), img_format) //11.73kb
        .expect("Couldn't write image to bytes.");

    let b64format = match img_format {
        ImageFormat::Jpeg => "jpeg",
        ImageFormat::Bmp => "bmp",
        _ => "png",
    };
    /*
    let b64 = format!(
        "![description](data:image/{};base64{})",
        b64format,
        BASE64_STANDARD.encode(bytes)
    );*/
    let b64 = format!(
        "data:image/{};base64,{}",
        b64format,
        BASE64_STANDARD.encode(bytes)
    );

    return b64;
}

/**
 * Export a `lib` function from Rust to JavaScript
 * @returns Jpeg base64 image with blur applied
 */
//
#[wasm_bindgen]
pub fn lib(base64_image: &str, blur_radius: Option<i32>) -> String {
    console_error_panic_hook::set_once();

    blur_radius.unwrap_or(5_i32); // default blur radius if not defined

    let base64_str = base64_image.split(',').nth(1).unwrap_or("");

    // Decode
    let base64_to_vector = BASE64_STANDARD.decode(base64_str).unwrap();
    println!("base64: {}", base64_str);

    let img = match load_from_memory_with_format(&base64_to_vector, ImageFormat::Jpeg) {
        Ok(img) => img.into_rgb8(),
        Err(_error) => unimplemented!(),
    };

    let blurred_image = gaussian_blur(&img, blur_radius.unwrap());

    let result = base64_save_image(blurred_image, ImageFormat::Jpeg);

    result
}

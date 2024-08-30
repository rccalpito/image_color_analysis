use image::{GenericImageView, Rgba};
use std::collections::HashMap;

fn main() {
    let img = image::open("./assets/IMG_8307.JPG").expect("Failed to open image");

    let mut color_counts: HashMap<Rgba<u8>, u32> = HashMap::new();

    for pixel in img.pixels() {
        let color = pixel.2;
        *color_counts.entry(color).or_insert(0) += 1;
    }

    let mut common_colors: Vec<(&Rgba<u8>, &u32)> = color_counts.iter().collect();
    common_colors.sort_by(|a, b| b.1.cmp(a.1)); // Sort by occurrence

    for (color, count) in common_colors.iter().take(100) {
        println!("Color {:?} occurs {} times", color, count);
    }

    let sorted_img = sort_image_by_color(img);

    sorted_img.save("sorted_image.png").expect("Failed to save image");
}

fn sort_image_by_color(img: image::DynamicImage) -> image::DynamicImage {
    let mut pixels: Vec<(u32, u32, Rgba<u8>)> = img.pixels().collect();

    pixels.sort_by_key(|&(_, _, color)| (color[0], color[1], color[2]));

    let (width, height) = img.dimensions();
    let mut sorted_img = image::ImageBuffer::new(width, height);

    for (i, pixel) in pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        sorted_img.put_pixel(x, y, pixel.2);
    }

    image::DynamicImage::ImageRgba8(sorted_img)
}
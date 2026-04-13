use image::GenericImageView;
use image::imageops::FilterType;
use std::fs;

const TERMINAL_WIDTH: u32 = 126;
const TERMINAL_HEIGHT: u32 = 32;

fn main() {
    let asci_map = fs::read_to_string("ascii_map.txt").expect("Couldn't read ASCII file");
    println!("ASCII map: {}", asci_map);
    let asci_length = asci_map.chars().count(); // 69 bokstäver
    println!("ASCII map character count: {}", asci_length);
    let mut _img = image::open("qwer.jpg").unwrap().grayscale();
    _img = image::DynamicImage::resize(&_img, TERMINAL_WIDTH, TERMINAL_HEIGHT, FilterType::Nearest);
    let gray = _img.to_luma8();
    let (width, height) = _img.dimensions();
    println!("{}, {}", width, height);

    //Hämtar en bild och gör den gråskalig.

    for y in 0..height {
        let mut pixels = Vec::new();
        for x in 0..width {
            let pixel = gray.get_pixel(x, y);
            let pixel_to_ascii = ((pixel[0] as f32 / 255.0) * 69.0).round() as u8;
            pixels.push(pixel_to_ascii);
        }
        print_picture(pixels, &asci_map);
    }
}

fn print_picture(pixels: Vec<u8>, ascii: &str) {
    let mut ascii_vector = Vec::new();
    for x in 0..pixels.len() {
        let ascii_value = ascii.chars().nth(pixels[x] as usize).unwrap();
        ascii_vector.push(ascii_value);
    }
    let joined_ascii_vector: String = ascii_vector.iter().map(|c| format!("{} ", c)).collect();
    println!("{}", joined_ascii_vector);
}

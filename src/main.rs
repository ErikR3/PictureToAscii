use crossterm::{
    ExecutableCommand,
    style::{Color, SetForegroundColor},
};
use image::GenericImageView;
use image::imageops::FilterType;
use std::fs;
use std::io::{Write, stdout};
use std::{thread, time::Duration};

const TERMINAL_WIDTH: u32 = 220;
const TERMINAL_HEIGHT: u32 = 40;

fn main() {
    let asci_map: String = fs::read_to_string("ascii_map.txt")
        .expect("Couldn't read ASCII file")
        .replace('\n', "")
        .replace('\r', "")
        .trim()
        .to_string();
    let asci_length = asci_map.chars().count(); // 69 bokstäver
    let mut _img = image::open("images.jpg").unwrap();
    _img = image::DynamicImage::resize(&_img, TERMINAL_WIDTH, TERMINAL_HEIGHT, FilterType::Nearest);
    // let gray = _img.to_luma8();
    let (width, height) = _img.dimensions();

    //Hämtar en bild och gör den gråskalig.

    for y in 0..height {
        let mut pixels = Vec::new();
        for x in 0..width {
            let pixel = _img.get_pixel(x, y);
            let brightness = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as f32;
            let ascii_index = ((brightness / 255.0) * (asci_length as f32 - 1.0)).round() as u8;
            let rgbb = vec![pixel[0], pixel[1], pixel[2], ascii_index];
            for z in 0..rgbb.len() {
                pixels.push(rgbb[z]);
            }
        }
        print_picture(pixels, &asci_map);
    }
}

fn print_picture(pixels: Vec<u8>, ascii: &str) {
    for chunk in pixels.chunks(4) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let ascii_index = chunk[3] as usize;
        let ascii_char = ascii.chars().nth(ascii_index).unwrap();
        let color = Color::Rgb { r, g, b };
        stdout().execute(SetForegroundColor(color)).unwrap();
        print!("{}{}", ascii_char, ascii_char);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(1));
    }
    println!("");
    stdout().execute(SetForegroundColor(Color::White)).unwrap();
}

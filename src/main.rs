use crossterm::{
    ExecutableCommand,
    style::{Color, SetForegroundColor},
};
use image::GenericImageView;
use image::imageops::FilterType;
use rand::prelude::*;
use std::fs;
use std::io::{Write, stdout};

const TERMINAL_WIDTH: u32 = 220;
const TERMINAL_HEIGHT: u32 = 22;

fn main() {
    let asci_map: String = fs::read_to_string("/home/erik/projects/PictureToAscii/ascii_map.txt")
        .expect("Couldn't read ASCII file")
        .replace('\n', "")
        .replace('\r', "")
        .trim()
        .to_string();
    let ascii_chars: Vec<char> = asci_map.chars().collect();

    let project_dir = "/home/erik/projects/PictureToAscii/";

    let pics: Vec<String> = fs::read_to_string("/home/erik/projects/PictureToAscii/rnd_pic.txt")
        .expect("Couldn't read rnd_pic.txt")
        .lines()
        .map(|l| format!("{}{}", project_dir, l.trim()))
        .collect();

    let chosen = &pics[rand::rng().random_range(0..pics.len())];

    let mut img = image::open(chosen).unwrap();
    img = img.resize(TERMINAL_WIDTH, TERMINAL_HEIGHT, FilterType::Nearest);
    let (width, height) = img.dimensions();

    for y in 0..height {
        let mut pixels = Vec::new();
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as f32;
            let ascii_index =
                ((brightness / 255.0) * (ascii_chars.len() as f32 - 1.0)).round() as u8;
            pixels.extend_from_slice(&[pixel[0], pixel[1], pixel[2], ascii_index]);
        }
        print_row(&pixels, &ascii_chars);
    }
    stdout().flush().unwrap();
}

fn print_row(pixels: &[u8], ascii: &[char]) {
    let mut buf = String::with_capacity(pixels.len() * 20);
    for chunk in pixels.chunks(4) {
        let (r, g, b) = (chunk[0], chunk[1], chunk[2]);
        let ascii_char = ascii[chunk[3] as usize];
        buf.push_str(&format!(
            "\x1B[38;2;{};{};{}m{}{}",
            r, g, b, ascii_char, ascii_char
        ));
    }
    buf.push_str("\x1B[0m\n");
    print!("{}", buf);
}

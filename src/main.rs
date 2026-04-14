use image::GenericImageView;
use image::{DynamicImage, RgbImage};
use std::fs;
use std::io::{Write, stdout};
use std::path::Path;
use std::{thread, time::Duration};
use video_rs::decode::Decoder;

const TERMINAL_WIDTH: u32 = 220;
const TERMINAL_HEIGHT: u32 = 50;

fn main() {
    let asci_map: String = fs::read_to_string("ascii_map.txt")
        .expect("Couldn't read ASCII file")
        .replace('\n', "")
        .replace('\r', "")
        .trim()
        .to_string();
    let ascii_chars: Vec<char> = asci_map.chars().collect();

    let mut decoder = Decoder::new(Path::new("pajaglajorna.mp4")).unwrap();
    let fps = decoder.frame_rate();
    let ms_per_frame = (1000.0 / fps) as u64;

    for frame in decoder.decode_iter() {
        let (_, frame) = frame.unwrap();
        let (h, w, _) = frame.dim();
        let data = frame.into_owned().into_raw_vec();
        let mut img =
            DynamicImage::ImageRgb8(RgbImage::from_raw(w as u32, h as u32, data).unwrap());
        img = img.resize(
            TERMINAL_WIDTH,
            TERMINAL_HEIGHT,
            image::imageops::FilterType::Nearest,
        );
        let (width, height) = img.dimensions();

        print!("\x1B[H");
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
        thread::sleep(Duration::from_millis(ms_per_frame));
    }
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

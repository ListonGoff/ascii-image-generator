#![allow(unused)]

use image;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgba};

const MAX_SIZE: u32 = 50;

fn main() {
    let _ascii_chars: [char; 30] = [
        ' ', '_', '.', ',', '-', '=', '+', ':', ';', 'c', 'b', 'a', '!', '?', '0', '1', '2', '3',
        '4', '5', '6', '7', '8', '9', '$', 'W', '#', '@', 'Ñ', '■',
    ];

    let mut img = image::open("Dog.png").expect("File not found!");

    //resize image so it isnt too big for console
    if img.height() > MAX_SIZE {
        img = img.resize(img.width(), MAX_SIZE, FilterType::Nearest);
    }

    if img.width() > MAX_SIZE {
        img = img.resize(MAX_SIZE, img.height(), FilterType::Nearest);
    }

    for y in 0..img.height() {
        for x in 0..img.width() {
            let rgb = img.get_pixel(x, y).0;
            let rgb_average = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as f32;

            let brightness = rgb_average / (255. / 30.);

            let char = _ascii_chars[(brightness - 1.0) as usize];
            print!("{} ", char)
        }
        println!();
    }
}

//for debug
fn print_pixel_info(img: &DynamicImage) {
    for pixel in img.pixels() {
        print!("Pos: ({}, {}); Rgba: (", pixel.0, pixel.1);

        for value in pixel.2 .0 {
            print!("{}, ", value)
        }
        print!("\x08\x08);\n");
    }
}

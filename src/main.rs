#![allow(unused)]

use image;
use image::{DynamicImage, GenericImageView, Rgba};
use image::imageops::FilterType;

fn main() {

    let _ascii_chars: [char; 30] = [
        ' ', '_', '.', ',', '-', '=', '+', ':', ';', 'c', 'b', 'a', '!', '?', '0', '1', '2', '3',
        '4', '5', '6', '7', '8', '9', '$', 'W', '#', '@', 'Ñ', '■'
    ];

    let mut img = image::open("Dog2.png").expect("File not found!");

    if img.height() > 100
    {
        img = img.resize(img.width(), 100, FilterType::Nearest);
    }

    if img.width() > 100
    {
        img = img.resize(100, img.height(), FilterType::Nearest);
    }

    let mut pixels = Vec::new();

    for pixel in img.pixels() {
        pixels.push(pixel);
    }

    for y in 0..img.height()
    {
        for x in 0..img.width()
        {
            let rgb = pixels[(x + (img.width() * y)) as usize].2.0;
            let rgb_average = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as f32;

            let brightness =  rgb_average / (255. / 30.);

            let char = _ascii_chars[(brightness - 1.0) as usize];
            print!("{} ", char)
        }
        println!();
    }
}
fn print_img(img: DynamicImage) {
    for pixel in img.pixels() {
        print!("Pos: ({}, {}); Rgba: (", pixel.0, pixel.1);

        for value in pixel.2 .0 {
            print!("{}, ", value)
        }
        print!("\x08\x08);\n");
    }
}

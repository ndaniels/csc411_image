use std::env;
pub use csc411_image::{Read, Write, RgbImage, GrayImage};

fn main() {
	let input = env::args().nth(1);
	let output = env::args().nth(2);
    let in_img: GrayImage = GrayImage::read(input.as_deref()).unwrap();
	let total: u32 = in_img.pixels.iter().fold(0, |acc, x| acc + x.value as u32);
	let res = total / (in_img.width * in_img.height);
	println!("{}", &res);
	GrayImage::write(&in_img, output.as_deref()).unwrap();
}
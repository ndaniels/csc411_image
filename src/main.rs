use std::env;

fn main() {
	let input = env::args().nth(1);
	let output = env::args().nth(2);
    let in_img = csc411_image::Image::read(input.as_deref()).unwrap();
    in_img.write(output.as_deref()).unwrap();
}
use std::fs;
use std::io::{self, BufRead, BufReader, Cursor};

use image::codecs::pnm;
use image::{DynamicImage, GenericImageView};

use crate::pixel::Gray;
use crate::pixel::Pixel;
use crate::pixel::Rgb;

/// A struct containing a vector of `Pixel`s,
/// a width, height, and denominator
#[derive(Clone)]
pub struct Image {
    /// The pixel values in row-major order, as a vec of `Pixel`
    /// Each pixel value is a scaled integer (Gray) or set of
    /// three integers (Rgb) in the range 0..denominator
    pub pixels: Vec<Pixel>,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// Denominator of the image
    pub denominator: u16,
}

impl Image {
    /// Reads an image from either a filename or stdin
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdin` is used
    pub fn read(filename: Option<&str>) -> Result<Self, String> {
        let mut raw_reader: Box<dyn BufRead> = match filename {
            None => Box::new(BufReader::new(io::stdin())),
            Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
        };
        let mut buffer = Vec::new();
        // read the whole contents
        raw_reader.read_to_end(&mut buffer).unwrap();
        let (mut cursor, header) = pnm::PnmDecoder::new(Cursor::new(&buffer[..]))
            .expect("Failed to read image format")
            .into_inner();
        // Rewind.
        cursor.set_position(0);
        let reader =
            image::codecs::pnm::PnmDecoder::new(cursor).expect("Failed to read image format");
        let img = DynamicImage::from_decoder(reader).unwrap();
        let pixels: Vec<Pixel> = match img {
            DynamicImage::ImageLuma8(_) => img
                .pixels()
                .map(|(_, _, p)| Pixel::Gray(Gray { value: p[0] as u16 }))
                .collect(),
            DynamicImage::ImageRgb8(_) => img
                .pixels()
                .map(|(_, _, p)| {
                    Pixel::Rgb(Rgb {
                        red: p[0] as u16,
                        green: p[1] as u16,
                        blue: p[2] as u16,
                    })
                })
                .collect(),
            _ => return Err("Unexpected image format".to_string()),
        };
        Ok(Image {
            pixels,
            width: img.width(),
            height: img.height(),
            denominator: header.maximal_sample() as u16,
        })
    }
}

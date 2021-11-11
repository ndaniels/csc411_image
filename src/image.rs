use image::ImageResult;
use image::codecs::pnm;
use image::pnm::{PNMSubtype, SampleEncoding};
use image::{DynamicImage, GenericImageView, ImageBuffer};

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
        let mut raw_reader: Box<dyn std::io::BufRead> = match filename {
            None => Box::new(std::io::BufReader::new(std::io::stdin())),
            Some(filename) => Box::new(std::io::BufReader::new(
                std::fs::File::open(filename).unwrap(),
            )),
        };
        let mut buffer = Vec::new();
        // read the whole contents
        raw_reader.read_to_end(&mut buffer).unwrap();
        let (mut cursor, header) = pnm::PnmDecoder::new(std::io::Cursor::new(&buffer[..]))
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

    /// Writes an image to either a filename or stdout
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdout` is used
    pub fn write(&self, filename: Option<&str>) -> ImageResult<()> {
        // we don't want to rely on file-extension magic,
        // so we should use write_to(&mut bytes, image::ImageOutputFormat::Pnm)
        // and apparently this should be wrapped in a BufWriter
        // We can rely on pixels being in row-major order,
        // and simply create a DynamicImage::ImageRgb8 whose
        // pixels are the appropriate values.
        // I do not believe we need to support writing of Pgm images,
        // but it shouldn't be a hard extension
        // (just pattern match on the type of the incoming pixels)

        let mut writer = match filename {
            Some(filename) => {
                let filename = std::path::Path::new(filename);
                Box::new(std::fs::File::create(&filename).unwrap()) as Box<dyn std::io::Write>
            }
            None => Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
        };
        let pixels = self
            .pixels
            .iter()
            .map(|p| match p {
                Pixel::Gray(gray) => vec![gray.value, gray.value, gray.value],
                Pixel::Rgb(rgb) => vec![rgb.red, rgb.green, rgb.blue],
            })
            .flatten()
            .map(|v| std::cmp::min(v, 255) as u8)
            .collect::<Vec<_>>();
        let img = ImageBuffer::from_vec(self.width, self.height, pixels).unwrap();
        let img = DynamicImage::ImageRgb8(img);
        img.write_to(
            &mut writer,
            image::ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary)),
        )
        // .map_err(|reason| format!("Failed to write image because {}", reason))
    }
}

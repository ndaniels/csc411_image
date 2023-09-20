use crate::imgtype::Gray;
use crate::imgtype::Rgb;
use image::codecs::pnm;
use image::pnm::{PNMSubtype, SampleEncoding};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::error::Error;

/// A struct containing a vector of RGB pixels,
/// a width, height, and denominator
#[derive(Debug)]
pub struct RgbImage {
    pub pixels: Vec<Rgb>,
    pub width: u32,
    pub height: u32,
    pub denominator: u16,
}

/// A struct containing a vector of Gray pixels,
/// a width, height, and denominator
#[derive(Debug)]
pub struct GrayImage {
    pub pixels: Vec<Gray>,
    pub width: u32,
    pub height: u32,
    pub denominator: u16,
}

/// Behavior that defines reading in a file
/// returns either a RGB or Gray Image
pub trait Read<T = Self> {
    fn read(filename: Option<&str>) -> Result<T, Box<dyn Error>>;
}

/// Behavior that defines writing an Image
// to the local filesystem
pub trait Write {
    fn write(&self, filename: Option<&str>) -> Result<(), Box<dyn Error>>;
}

impl Read for RgbImage {
    /// Reads an RgbImage from either a filename or stdin
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdin` must contain pgm data
    fn read(filename: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let mut raw_reader: Box<dyn std::io::BufRead> = match filename {
            None => Box::new(std::io::BufReader::new(std::io::stdin())),
            Some(filename) => Box::new(std::io::BufReader::new(std::fs::File::open(filename)?)),
        };
        let mut buffer = Vec::new();
        // read the whole contents
        raw_reader.read_to_end(&mut buffer)?;
        let (mut cursor, header) = pnm::PnmDecoder::new(std::io::Cursor::new(&buffer[..]))
            .expect("Failed to read image format")
            .into_inner();
        // Rewind.
        cursor.set_position(0);
        let reader =
            image::codecs::pnm::PnmDecoder::new(cursor).expect("Failed to read image format");
        let img = DynamicImage::from_decoder(reader)?;
        let pixels: Vec<Rgb> = match img {
            DynamicImage::ImageRgb8(_) => img
                .pixels()
                .map(|(_, _, p)| Rgb {
                    red: p[0] as u16,
                    green: p[1] as u16,
                    blue: p[2] as u16,
                })
                .collect(),
            _ => return Err("Unexpected image format".to_string().into()),
        };
        Ok(RgbImage {
            pixels,
            width: img.width(),
            height: img.height(),
            denominator: header.maximal_sample() as u16,
        })
    }
}

impl Write for RgbImage {
    /// Writes an RgbImage to either a filename or stdout
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdout` is used
    fn write(&self, filename: Option<&str>) -> Result<(), Box<dyn Error>> {
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
                Box::new(std::fs::File::create(filename)?) as Box<dyn std::io::Write>
            }
            None => Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
        };

        let pixels = self
            .pixels
            .iter()
            .flat_map(|p| vec![p.red, p.green, p.blue])
            .map(|v| std::cmp::min(v, 255) as u8)
            .collect::<Vec<_>>();
        let img = ImageBuffer::from_vec(self.width, self.height, pixels)
            .ok_or("Insufficient buffer size")?;
        let img = DynamicImage::ImageRgb8(img);
        img.write_to(
            &mut writer,
            image::ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary)),
        )
        .map_err(|reason| format!("Failed to write image because {reason}").into())
    }
}

impl Read for GrayImage {
    /// Reads an GrayImage from either a filename or stdin
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdin` must contain pgm data
    fn read(filename: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let mut raw_reader: Box<dyn std::io::BufRead> = match filename {
            None => Box::new(std::io::BufReader::new(std::io::stdin())),
            Some(filename) => Box::new(std::io::BufReader::new(std::fs::File::open(filename)?)),
        };
        let mut buffer = Vec::new();
        // read the whole contents
        raw_reader.read_to_end(&mut buffer)?;
        let (mut cursor, header) = pnm::PnmDecoder::new(std::io::Cursor::new(&buffer[..]))
            .expect("Failed to read image format")
            .into_inner();
        // Rewind.
        cursor.set_position(0);
        let reader =
            image::codecs::pnm::PnmDecoder::new(cursor).expect("Failed to read image format");
        let img = DynamicImage::from_decoder(reader)?;
        let pixels: Vec<Gray> = match img {
            DynamicImage::ImageLuma8(_) => img
                .pixels()
                .map(|(_, _, p)| Gray { value: p[0] as u16 })
                .collect(),
            DynamicImage::ImageRgb8(_) => img
                .pixels()
                .map(|(_, _, p)| Gray {
                    value: (p[0] as u16 + p[1] as u16 + p[2] as u16) / 3_u16,
                })
                .collect(),
            _ => return Err("Unexpected image format".to_string().into()),
        };
        Ok(GrayImage {
            pixels,
            width: img.width(),
            height: img.height(),
            denominator: header.maximal_sample() as u16,
        })
    }
}

impl Write for GrayImage {
    /// Writes a GrayImage to either a filename or stdout
    ///
    /// # Arguments
    ///
    /// * `filename`: a string containing a path to an image file,
    ///                 or `None`, in which case `stdout` is used
    fn write(&self, filename: Option<&str>) -> Result<(), Box<dyn Error>> {
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
                Box::new(std::fs::File::create(filename)?) as Box<dyn std::io::Write>
            }
            None => Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
        };

        let pixels = self
            .pixels
            .iter()
            .flat_map(|p| vec![p.value, p.value, p.value])
            .map(|v| std::cmp::min(v, 255) as u8)
            .collect::<Vec<_>>();
        let img = ImageBuffer::from_vec(self.width, self.height, pixels)
            .ok_or("Insufficient buffer size")?;
        let img = DynamicImage::ImageRgb8(img);
        img.write_to(
            &mut writer,
            image::ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary)),
        )
        .map_err(|reason| format!("Failed to write image because {reason}").into())
    }
}

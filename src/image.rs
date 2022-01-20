use image::codecs::pnm;
use image::pnm::{PNMSubtype, SampleEncoding};
use image::{DynamicImage, GenericImageView, ImageBuffer };
use crate::imgtype::Gray;
use crate::imgtype::Rgb;

#[derive(Debug)]
pub struct RgbImage{
    pub pixels: Vec<Rgb>,
    pub width: u32,
    pub height: u32,
    pub denominator: u16
}
#[derive(Debug)]
pub struct GrayImage{
    pub pixels: Vec<Gray>,
    pub width: u32,
    pub height: u32,
    pub denominator: u16
}

pub trait Read<T=Self>{
    fn read(filename: Option<&str>) -> Result<T, String>;
}

pub trait Write{
    fn write(&self, filename: Option<&str>) -> Result<(), String>;
}

impl Read for RgbImage{
    fn read(filename: Option<&str>) -> Result<Self, String> {
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
        let pixels: Vec<Rgb> = match img {
            DynamicImage::ImageRgb8(_) => img
                .pixels()
                .map(|(_, _, p)| {
                    Rgb {
                        red: p[0] as u16,
                        green: p[1] as u16,
                        blue: p[2] as u16,
                    }
                })
                .collect(),
            _ => return Err("Unexpected image format".to_string()),
        };
        Ok(RgbImage{
            pixels,
            width: img.width(),
            height: img.height(),
            denominator: header.maximal_sample() as u16
        })
    }
}

impl Write for RgbImage {
    fn write(&self, filename: Option<&str>) -> Result<(), String> {
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
            .map(|p|  {
                vec![ p.red, p.green, p.blue]
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
        .map_err(|reason| format!("Failed to write image because {}", reason))
    }
}


impl Read for GrayImage{
    fn read(filename: Option<&str>) -> Result<Self, String> {
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
        let pixels: Vec<Gray> = match img {
            DynamicImage::ImageLuma8(_) => img
            .pixels()
            .map(|(_, _, p)| Gray { value: p[0] as u16 })
            .collect(),
            DynamicImage::ImageRgb8(_) => img
                .pixels()
                .map(|(_, _, p)| {
                    Gray {
                       value: (p[0] as u16 + p[1] as u16 + p[2] as u16) / 3_u16,
                    }
                })
                .collect(),
            _ => return Err("Unexpected image format".to_string()),
        };
        Ok(GrayImage{
            pixels,
            width: img.width(),
            height: img.height(),
            denominator: header.maximal_sample() as u16
        })
    }
}
impl Write for GrayImage {

    fn write(&self, filename: Option<&str>) -> Result<(), String> {
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
            .map(|p|  {
                vec![ p.value, p.value, p.value]
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
        .map_err(|reason| format!("Failed to write image because {}", reason))
    }
}

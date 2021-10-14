mod image;
mod pixel;
pub use crate::image::Image;
pub use crate::pixel::{Gray, Pixel, Rgb};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

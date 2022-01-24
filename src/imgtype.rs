/// Public enumerator to specify the type of the Image being built
pub enum ImageType {
    Gray(Gray),
    Rgb(Rgb)
}

/// A `Gray` pixel contains a single `u16` value indicating brightness
#[derive(Clone, Debug)]
pub struct Gray {
    pub value: u16,
}

/// An `Rgb` pixel contains three `u16` values, for red, green, and blue channels respectively
#[derive(Clone, Debug)]
pub struct Rgb {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
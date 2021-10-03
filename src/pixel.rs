pub enum Pixel {
    Gray(Gray),
    Rgb(Rgb),
}

pub struct Gray {
    pub value: u16,
}

pub struct Rgb {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
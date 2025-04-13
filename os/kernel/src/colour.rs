#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Colour {
    value: u32,
}

impl Colour {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            value: u32::from_be_bytes([0xff, red, green, blue]),
        }
    }

    pub const WHITE: Colour = Colour::from_rgb(0xff, 0xff, 0xff);

    pub const RED: Colour = Colour::from_rgb(0xff, 0, 0);
    pub const GREEN: Colour = Colour::from_rgb(0, 0xff, 0);
    pub const BLUE: Colour = Colour::from_rgb(0, 0, 0xff);

    pub const CYAN: Colour = Colour::from_rgb(0, 0xff, 0xff);
    pub const MAGENTA: Colour = Colour::from_rgb(0xff, 0, 0xff);
    pub const YELLOW: Colour = Colour::from_rgb(0xff, 0xff, 0);

    pub const BLACK: Colour = Colour::from_rgb(0, 0, 0);
}

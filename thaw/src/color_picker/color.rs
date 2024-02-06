use palette::{Hsl, Hsv, Srgb};

#[derive(Clone)]
pub enum Color {
    RGB(Srgb),
    HSV(Hsv),
    HSL(Hsl),
}

impl Default for Color {
    fn default() -> Self {
        Self::RGB(Srgb::new(0.0, 0.0, 0.0))
    }
}

impl From<Srgb> for Color {
    fn from(value: Srgb) -> Self {
        Self::RGB(value)
    }
}

impl From<Hsv> for Color {
    fn from(value: Hsv) -> Self {
        Self::HSV(value)
    }
}

impl From<Hsl> for Color {
    fn from(value: Hsl) -> Self {
        Self::HSL(value)
    }
}

#[derive(Clone, Copy)]
pub struct RGB {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8
}

impl RGB {
    #[inline]
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB { red, green, blue }
    }
}

impl std::fmt::Debug for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Red: {}; Blue: {}; Green: {}", self.red, self.green, self.blue)
    }
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
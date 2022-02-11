use crate::math::general::IsInRange;

const ARG_ERR: &str = "Arguments must range from 0 to 1.";
pub struct CMYK {
    cyan:    f32,
    magenta: f32,
    yellow:  f32,
    black:   f32
}

impl CMYK {
    pub fn new(cyan:    f32,
               magenta: f32,
               yellow:  f32,
               black:   f32) -> CMYK {
        if !cyan.is_in_range(0.0, 1.0) ||
           !magenta.is_in_range(0.0, 1.0) ||
           !yellow.is_in_range(0.0, 1.0) ||
           !black.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        CMYK { cyan, magenta, yellow, black }
    }

    pub fn cyan(&self) -> &f32 {
        &self.cyan
    }

    pub fn magenta(&self) -> &f32 {
        &self.magenta
    }

    pub fn yellow(&self) -> &f32 {
        &self.yellow
    }

    pub fn black(&self) -> &f32 {
        &self.black
    }

    pub fn set_cyan(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.cyan = value;
    }

    pub fn set_magenta(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.magenta = value;
    }

    pub fn set_yellow(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.yellow = value;
    }

    pub fn set_black(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.black = value;
    }
}
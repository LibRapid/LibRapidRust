use crate::math::general::IsInRange;

const ARG_ERR: &str = "Arguments must range from 0 to 1.";
#[derive(Clone, Copy)]
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
    #[inline]
    #[must_use]
    pub fn cyan(&self) -> &f32 {
        &self.cyan
    }
    #[inline]
    #[must_use]
    pub fn magenta(&self) -> &f32 {
        &self.magenta
    }
    #[inline]
    #[must_use]
    pub fn yellow(&self) -> &f32 {
        &self.yellow
    }
    #[inline]
    #[must_use]
    pub fn black(&self) -> &f32 {
        &self.black
    }
    #[inline]
    #[must_use]
    pub fn set_cyan(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.cyan = value;
    }
    #[inline]
    #[must_use]
    pub fn set_magenta(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.magenta = value;
    }
    #[inline]
    #[must_use]
    pub fn set_yellow(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.yellow = value;
    }
    #[inline]
    #[must_use]
    pub fn set_black(&mut self, value: f32) {
        if !value.is_in_range(0.0, 1.0)
        { panic!("{}", ARG_ERR); }

        self.black = value;
    }
}
//! Definition of a chemical element.
pub mod pse;
/// The struct for storing a chemical element.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Element<'a> {
    pub symbol:           &'a str,
    pub protons:          u8,
    pub electrons:        u8,
    pub neutrons:         u8,
    pub atomic_weight:    f32,
    pub pauling_en:       f32
}

impl<'a> Element<'a> {
    /// Create a new, custom chemical element.
    /// # Arguments
    /// * `symbol: &str` - The element symbol.
    /// * `protons: u8` - The number of protons.
    /// * `electrons: u8` - The number of electrons.
    /// * `neutrons: u8` - The number of neutrons.
    /// * `atomic_weight: f32` - The weighted average of all isotopes.
    /// * `pauling_en: f32` - The electronegativity after Pauling.
    /// # Examples
    /// ```
    /// use lib_rapid::chem::elements::Element;
    /// 
    /// let elem = Element::new("Xy", 255, 255, 255, 510.7, 6.54); // Fictional element.
    /// assert_eq!(elem.atomic_weight, 510.7);
    /// ```
    pub const fn new(symbol:           &str,
                     protons:          u8,
                     electrons:        u8,
                     neutrons:         u8,
                     atomic_weight:    f32,
                     pauling_en:       f32) -> Element {
        Element { symbol, protons, electrons, neutrons, atomic_weight, pauling_en }
    }
    /// Lets you get the mass number of a direct element.
    /// # Examples
    /// ```
    /// use lib_rapid::chem::elements::Element;
    /// 
    /// let elem = Element::new("Xy", 255, 255, 255, 510.7, 6.54); // Fictional element.
    /// assert_eq!(elem.mass_number(), 510);
    /// ```
    pub const fn mass_number(&self) -> u16 {
        self.protons as u16 + self.neutrons as u16
    }
}
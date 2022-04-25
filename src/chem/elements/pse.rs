//! Chemical constants from the perodic system of elements.
use super::Element;
/// The chemical element Hydrogen.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::HYDROGEN;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const HYDROGEN:      Element = Element::new("H",  1,   1,   0,   1.008,  2.20);
/// The chemical element Helium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::HELIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const HELIUM:        Element = Element::new("He", 2,   2,   2,   4.0026, f32::NAN);
/// The chemical element Lithium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LITHIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LITHIUM:       Element = Element::new("Li", 3,   3,   4,   6.94,   0.98);
/// The chemical element Beryllium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BERYLLIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BERYLLIUM:     Element = Element::new("Be", 4,   4,   5,   9.0122, 1.57);
/// The chemical element Boron.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BORON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BORON:         Element = Element::new("B",  5,   5,   6,   10.81,  2.04);
/// The chemical element Carbon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CARBON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CARBON:        Element = Element::new("C",  6,   6,   6,   12.011, 2.55);
/// The chemical element Nitrogen.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NITROGEN;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NITROGEN:      Element = Element::new("N",  7,   7,   7,   14.007, 3.04);
/// The chemical element Oxygen.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::OXYGEN;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const OXYGEN:        Element = Element::new("O",  8,   8,   8,   15.999, 3.44);
/// The chemical element Fluorine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::FLUORINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const FLUORINE:      Element = Element::new("F",  9,   9,   10,  18.998, 3.98);
/// The chemical element Neon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NEON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NEON:          Element = Element::new("Ne", 10,  10,  10,  20.180, f32::NAN);
/// The chemical element Sodium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SODIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SODIUM:        Element = Element::new("Na", 11,  11,  12,  22.990, 0.93);
/// The chemical element Magnesium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MAGNESIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MAGNESIUM:     Element = Element::new("Mg", 12,  12,  12,  24.305, 1.31);
/// The chemical element Aluminium (No, not `aluminum`).
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ALUMINIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ALUMINIUM:     Element = Element::new("Al", 13,  13,  14,  26.982, 1.61);
/// The chemical element Silicon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SILICON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SILICON:       Element = Element::new("Si", 14,  14,  14,  28.085, 1.90);
/// The chemical element Phosphorus.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PHOSPHORUS;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PHOSPHORUS:    Element = Element::new("P",  15,  15,  16,  30.974, 2.19);
/// The chemical element Sulfur.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SULFUR;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SULFUR:        Element = Element::new("S",  16,  16,  16,  32.06,  2.58);
/// The chemical element Chlorine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CHLORINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CHLORINE:      Element = Element::new("Cl", 17,  17,  18,  35.45,  3.16);
/// The chemical element Argon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ARGON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ARGON:         Element = Element::new("Ar", 18,  18,  22,  39.95,  f32::NAN);
/// The chemical element Potassium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::POTASSIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const POTASSIUM:     Element = Element::new("K",  19,  19,  20,  39.098, 0.82);
/// The chemical element Calcium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CALCIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CALCIUM:       Element = Element::new("Ca", 20,  20,  20,  40.078, 1.00);
/// The chemical element Scandium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SCANDIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SCANDIUM:      Element = Element::new("Sc", 21,  21,  24,  44.956, 1.36);
/// The chemical element Titanium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TITANIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TITANIUM:      Element = Element::new("Ti", 22,  22,  26,  47.867, 1.54);
/// The chemical element Vanadium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::VANADIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const VANADIUM:      Element = Element::new("V",  23,  23,  28,  50.942, 1.63);
/// The chemical element Chromium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CHROMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CHROMIUM:      Element = Element::new("Cr", 24,  24,  28,  51.996, 1.66);
/// The chemical element Manganese.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MANGANESE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MANGANESE:     Element = Element::new("Mn", 25,  25,  30,  54.938, 1.55);
/// The chemical element Iron.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::IRON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const IRON:          Element = Element::new("Fe", 26,  26,  30,  55.845, 1.83);
/// The chemical element Cobalt.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::COBALT;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const COBALT:        Element = Element::new("Co", 27,  27,  32,  58.933, 1.88);
/// The chemical element Nickel.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NICKEL;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NICKEL:        Element = Element::new("Ni", 28,  28,  31,  58.693, 1.91);
/// The chemical element Copper.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::COPPER;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const COPPER:        Element = Element::new("Cu", 29,  29,  35,  63.546, 1.90);
/// The chemical element Zinc.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ZINC;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ZINC:          Element = Element::new("Zn", 30,  30,  35,  65.38,  1.65);
/// The chemical element Gallium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::GALLIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const GALLIUM:       Element = Element::new("Ga", 31,  31,  39,  69.723, 1.81);
/// The chemical element Germanium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::GERMANIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const GERMANIUM:     Element = Element::new("Ge", 32,  32,  41,  72.630, 2.01);
/// The chemical element Arsenic.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ARSENIC;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ARSENIC:       Element = Element::new("As", 33,  33,  42,  74.922, 2.18);
/// The chemical element Selenium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SELENIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SELENIUM:      Element = Element::new("Se", 34,  34,  45,  78.971, 2.55);
/// The chemical element Bromine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BROMINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BROMINE:       Element = Element::new("Br", 35,  35,  45,  79.904, 2.96);
/// The chemical element Krypton.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::KRYPTON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const KRYPTON:       Element = Element::new("Kr", 36,  36,  48,  83.798, 3.00);
/// The chemical element Rubidium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RUBIDIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RUBIDIUM:      Element = Element::new("Rb", 37,  37,  48,  85.468, 0.82);
/// The chemical element Strontium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::STRONTIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const STRONTIUM:     Element = Element::new("Sr", 38,  38,  50,  87.62,  0.95);
/// The chemical element Yttrium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::YTTRIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const YTTRIUM:       Element = Element::new("Y",  39,  39,  50,  88.906, 1.22);
/// The chemical element Zirconium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ZIRCONIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ZIRCONIUM:     Element = Element::new("Zr", 40,  40,  51,  91.224, 1.33);
/// The chemical element Niobium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NIOBIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NIOBIUM:       Element = Element::new("Nb", 41,  41,  52,  92.906, 1.60);
/// The chemical element Molybdenum.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MOLYBDENUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MOLYBDENUM:    Element = Element::new("Mo", 42,  42,  54,  95.95,  2.16);
/// The chemical element Technetium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TECHNETIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TECHNETIUM:    Element = Element::new("Tc", 43,  43,  54,  97.0,   1.90);
/// The chemical element Ruthenium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RUTHENIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RUTHENIUM:     Element = Element::new("Ru", 44,  44,  57,  101.07, 2.20);
/// The chemical element Rhodium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RHODIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RHODIUM:       Element = Element::new("Rh", 45,  45,  58,  102.91, 2.28);
/// The chemical element Palladium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PALLADIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PALLADIUM:     Element = Element::new("Pd", 46,  46,  60,  106.42, 2.20);
/// The chemical element Silver.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SILVER;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SILVER:        Element = Element::new("Ag", 47,  47,  61,  107.87, 1.93);
/// The chemical element Cadmium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CADMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CADMIUM:       Element = Element::new("Cd", 48,  48,  64,  112.41, 1.69);
/// The chemical element Indium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::INDIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const INDIUM:        Element = Element::new("In", 49,  49,  66,  114.82, 1.78);
/// The chemical element Tin.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TIN;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TIN:           Element = Element::new("Sn", 50,  50,  69,  118.71, 1.96);
/// The chemical element Antimony.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ANTIMONY;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ANTIMONY:      Element = Element::new("Sb", 51,  51,  71,  121.76, 2.05);
/// The chemical element Tellurium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TELLURIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TELLURIUM:     Element = Element::new("Te", 52,  52,  76,  127.60, 2.10);
/// The chemical element Iodine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::IODINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const IODINE:        Element = Element::new("I",  53,  53,  74,  126.90, 2.66);
/// The chemical element Xenon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::XENON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const XENON:         Element = Element::new("Xe", 54,  54,  77,  131.29, 2.60);
/// The chemical element Caesium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CAESIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CAESIUM:       Element = Element::new("Cs", 55,  55,  78,  132.91, 0.79);
/// The chemical element Barium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BARIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BARIUM:        Element = Element::new("Ba", 56,  56,  81,  137.33, 0.89);
/// The chemical element Lanthanum.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LANTHANUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LANTHANUM:     Element = Element::new("La", 57,  57,  82,  138.91, 1.10);
/// The chemical element Cerium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CERIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CERIUM:        Element = Element::new("Ce", 58,  58,  82,  140.12, 1.12);
/// The chemical element Praseodymium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PRASEODYMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PRASEODYMIUM:  Element = Element::new("Pr", 59,  59,  82,  140.91, 1.13);
/// The chemical element Neodymium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NEODYMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NEODYMIUM:     Element = Element::new("Nd", 60,  60,  84,  144.24, 1.14);
/// The chemical element Promethium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PROMETHIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PROMETHIUM:    Element = Element::new("Pm", 61,  61,  84,  145.0,  1.13);
/// The chemical element Samarium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SAMARIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SAMARIUM:      Element = Element::new("Sm", 62,  62,  88,  150.36, 1.17);
/// The chemical element Europium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::EUROPIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const EUROPIUM:      Element = Element::new("Eu", 63,  63,  89,  151.96, 1.20);
/// The chemical element Gadolinium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::GADOLINIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const GADOLINIUM:    Element = Element::new("Gd", 64,  64,  93,  157.25, 1.20);
/// The chemical element Terbium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TERBIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TERBIUM:       Element = Element::new("Tb", 65,  65,  94,  158.93, 1.20);
/// The chemical element Dysprosium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::DYSPROSIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const DYSPROSIUM:    Element = Element::new("Dy", 66,  66,  97,  162.50, 1.22);
/// The chemical element Holmium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::HOLMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const HOLMIUM:       Element = Element::new("Ho", 67,  67,  98,  164.93, 1.23);
/// The chemical element Erbium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ERBIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ERBIUM:        Element = Element::new("Er", 68,  68,  99,  167.26, 1.24);
/// The chemical element Thulium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::THULIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const THULIUM:       Element = Element::new("Tm", 69,  69,  100, 168.93, 1.25);
/// The chemical element Ytterbium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::YTTERBIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const YTTERBIUM:     Element = Element::new("Yb", 70,  70,  103, 173.05, 1.10);
/// The chemical element Lutetium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LUTETIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LUTETIUM:      Element = Element::new("Lu", 71,  71,  104, 174.97, 1.27);
/// The chemical element Hafnium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::HAFNIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const HAFNIUM:       Element = Element::new("Hf", 72,  72,  106, 178.49, 1.30);
/// The chemical element Tantalum.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TANTALUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TANTALUM:      Element = Element::new("Ta", 73,  73,  108, 180.95, 1.50);
/// The chemical element Tungsten.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TUNGSTEN;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TUNGSTEN:      Element = Element::new("W",  74,  74,  110, 183.84, 2.36);
/// The chemical element Rhenium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RHENIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RHENIUM:       Element = Element::new("Re", 75,  75,  111, 186.21, 1.90);
/// The chemical element Osmium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::OSMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const OSMIUM:        Element = Element::new("Os", 76,  76,  114, 190.23, 2.20);
/// The chemical element Iridium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::IRIDIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const IRIDIUM:       Element = Element::new("Ir", 77,  77,  115, 192.22, 2.20);
/// The chemical element Platinum.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PLATINUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PLATINUM:      Element = Element::new("Pt", 78,  78,  117, 195.08, 2.28);
/// The chemical element Gold.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::GOLD;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const GOLD:          Element = Element::new("Au", 79,  79,  118, 196.97, 2.54);
/// The chemical element Mercury.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MERCURY;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MERCURY:       Element = Element::new("Hg", 80,  80,  121, 200.59, 2.00);
/// The chemical element Thallium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::THALLIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const THALLIUM:      Element = Element::new("Tl", 81,  81,  123, 204.38, 1.62);
/// The chemical element Lead.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LEAD;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LEAD:          Element = Element::new("Pb", 82,  82,  125, 207.2,  2.33);
/// The chemical element Bismuth.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BISMUTH;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BISMUTH:       Element = Element::new("Bi", 83,  83,  126, 208.98, 2.02);
/// The chemical element Polonium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::POLONIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const POLONIUM:      Element = Element::new("Po", 84,  84,  125, 209.0,  2.00);
/// The chemical element Astatine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ASTATINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ASTATINE:      Element = Element::new("At", 85,  85,  125, 210.0,  2.20);
/// The chemical element Radon.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RADON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RADON:         Element = Element::new("Rn", 86,  86,  136, 222.0,  2.20);
/// The chemical element Francium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::FRANCIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const FRANCIUM:      Element = Element::new("Fr", 87,  87,  136, 223.0,  0.79);
/// The chemical element Radium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RADIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RADIUM:        Element = Element::new("Ra", 88,  88,  138, 226.0,  0.90);
/// The chemical element Actinium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ACTINIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ACTINIUM:      Element = Element::new("Ac", 89,  89,  138, 227.0,  1.10);
/// The chemical element Thorium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::THORIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const THORIUM:       Element = Element::new("Th", 90,  90,  142, 232.04, 1.30);
/// The chemical element Protactinium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PROTACTINIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PROTACTINIUM:  Element = Element::new("Pa", 91,  91,  140, 231.04, 1.50);
/// The chemical element Uranium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::URANIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const URANIUM:       Element = Element::new("U",  92,  92,  146, 238.03, 1.38);
/// The chemical element Neptunium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NEPTUNIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NEPTUNIUM:     Element = Element::new("Np", 93,  93,  144, 237.0,  1.36);
/// The chemical element Plutonium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::PLUTONIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const PLUTONIUM:     Element = Element::new("Pu", 94,  94,  150, 244.0,  1.28);
/// The chemical element Americium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::AMERICIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const AMERICIUM:     Element = Element::new("Am", 95,  95,  148, 243.0,  1.13);
/// The chemical element Curium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CURIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CURIUM:        Element = Element::new("Cm", 96,  96,  151, 247.0,  1.28);
/// The chemical element Berkelium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BERKELIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BERKELIUM:     Element = Element::new("Bk", 97,  97,  150, 247.0,  1.30);
/// The chemical element Californium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::CALIFORNIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const CALIFORNIUM:   Element = Element::new("Cf", 98,  98,  153, 251.0,  1.30);
/// The chemical element Einsteinium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::EINSTEINIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const EINSTEINIUM:   Element = Element::new("Es", 99,  99,  153, 252.0,  1.30);
/// The chemical element Fermium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::FERMIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const FERMIUM:       Element = Element::new("Fm", 100, 100, 157, 257.0,  1.30);
/// The chemical element Mendelevium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MENDELEVIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MENDELEVIUM:   Element = Element::new("Md", 101, 101, 157, 258.0,  1.30);
/// The chemical element Nobelium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NOBELIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NOBELIUM:      Element = Element::new("No", 102, 102, 157, 259.0,  1.30);
/// The chemical element LAWRENCIUM.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LAWRENCIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LAWRENCIUM:    Element = Element::new("Lr", 103, 103, 163, 266.0,  1.30);
/// The chemical element Rutherfordium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::RUTHERFORDIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const RUTHERFORDIUM: Element = Element::new("Rf", 104, 104, 163, 267.0,  f32::NAN);
/// The chemical element Dubnium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::DUBNIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const DUBNIUM:       Element = Element::new("Db", 105, 105, 163, 268.0,  f32::NAN);
/// The chemical element Seaborgium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::SEABORGIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const SEABORGIUM:    Element = Element::new("Sg", 106, 106, 163, 269.0,  f32::NAN);
/// The chemical element Bohrium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::BOHRIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const BOHRIUM:       Element = Element::new("Bh", 107, 107, 163, 270.0,  f32::NAN);
/// The chemical element Hassium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::HASSIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const HASSIUM:       Element = Element::new("Hs", 108, 108, 161, 269.0,  f32::NAN);
/// The chemical element Meitnerium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MEITNERIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MEITNERIUM:    Element = Element::new("Mt", 109, 109, 169, 278.0,  f32::NAN);
/// The chemical element Darmstadtium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::DARMSTADTIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const DARMSTADTIUM:  Element = Element::new("Ds", 110, 110, 171, 281.0,  f32::NAN);
/// The chemical element Roentgenium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::ROENTGENIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const ROENTGENIUM:   Element = Element::new("Rg", 111, 111, 171, 282.0,  f32::NAN);
/// The chemical element Copernicium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::COPERNICIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const COPERNICIUM:   Element = Element::new("Cn", 112, 112, 173, 285.0,  f32::NAN);
/// The chemical element Nihonium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::NIHONIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const NIHONIUM:      Element = Element::new("Nh", 113, 113, 173, 286.0,  f32::NAN);
/// The chemical element Flerovium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::FLEROVIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const FLEROVIUM:     Element = Element::new("Fl", 114, 114, 175, 289.0,  f32::NAN);
/// The chemical element Moscovium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::MOSCOVIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const MOSCOVIUM:     Element = Element::new("Mc", 115, 115, 175, 290.0,  f32::NAN);
/// The chemical element Livermorium.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::LIVERMORIUM;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const LIVERMORIUM:   Element = Element::new("Lv", 116, 116, 177, 293.0,  f32::NAN);
/// The chemical element Tennessine.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::TENNESSINE;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const TENNESSINE:    Element = Element::new("Ts", 117, 117, 177, 294.0,  f32::NAN);
/// The chemical element Oganesson.
/// ```
/// use lib_rapid::chem::elements::pse;
///
/// let element = pse::OGANESSON;
/// assert_eq!(element.atomic_weight.round() - element.protons as f32, element.neutrons as f32);
/// ```
pub const OGANESSON:     Element = Element::new("Og", 118, 118, 176, 294.0,  f32::NAN);

/// All known chemical elements ordered by the atomic number.
/// ```
/// use lib_rapid::chem::elements::pse::*;
/// 
/// assert_eq!(PSE[78], GOLD);
/// ```
pub const PSE:           [Element; 118] = [ HYDROGEN, HELIUM, LITHIUM, BERYLLIUM, BORON, CARBON, NITROGEN, OXYGEN, FLUORINE, NEON, SODIUM, MAGNESIUM, ALUMINIUM, SILICON, PHOSPHORUS, SULFUR, CHLORINE, ARGON, POTASSIUM, CALCIUM, SCANDIUM, TITANIUM, VANADIUM, CHROMIUM, MANGANESE, IRON, COBALT, NICKEL, COPPER, ZINC, GALLIUM, GERMANIUM, ARSENIC, SELENIUM, BROMINE, KRYPTON, RUBIDIUM, STRONTIUM, YTTRIUM, ZIRCONIUM, NIOBIUM, MOLYBDENUM, TECHNETIUM, RUTHENIUM, RHODIUM, PALLADIUM, SILVER, CADMIUM, INDIUM, TIN, ANTIMONY, TELLURIUM, IODINE, XENON, CAESIUM, BARIUM, LANTHANUM, CERIUM, PRASEODYMIUM, NEODYMIUM, PROMETHIUM, SAMARIUM, EUROPIUM, GADOLINIUM, TERBIUM, DYSPROSIUM, HOLMIUM, ERBIUM, THULIUM, YTTERBIUM, LUTETIUM, HAFNIUM, TANTALUM, TUNGSTEN, RHENIUM, OSMIUM, IRIDIUM, PLATINUM, GOLD, MERCURY, THALLIUM, LEAD, BISMUTH, POLONIUM, ASTATINE, RADON, FRANCIUM, RADIUM, ACTINIUM, THORIUM, PROTACTINIUM, URANIUM, NEPTUNIUM, PLUTONIUM, AMERICIUM, CURIUM, BERKELIUM, CALIFORNIUM, EINSTEINIUM, FERMIUM, MENDELEVIUM, NOBELIUM, LAWRENCIUM, RUTHERFORDIUM, DUBNIUM, SEABORGIUM, BOHRIUM, HASSIUM, MEITNERIUM, DARMSTADTIUM, ROENTGENIUM, COPERNICIUM, NIHONIUM, FLEROVIUM, MOSCOVIUM, LIVERMORIUM, TENNESSINE, OGANESSON ];

/// The mass of one electron in Dalton.
pub const ELECTRONMASS: f64 = 5.4857990907016e-4;
/// The mass of one proton in Dalton.
pub const PROTONMASS:   f64 = 1.00727646662153;
/// The mass of one neutron in Dalton.
pub const NEUTRONMASS:  f64 = 1.0086649158849;

/// Get a chemical element by its atomic number.
/// # Arguments
/// * `n: u8` the atomic number.
/// # Examples
/// ```
/// use lib_rapid::chem::elements::pse::*;
/// 
/// assert_eq!(elem_by_a_num(79), GOLD);
/// ```
pub const fn elem_by_a_num<'a>(n: u8) -> Element<'a> {
    PSE[(n - 1) as usize]
}
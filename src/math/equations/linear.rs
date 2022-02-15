#[derive(Clone, Copy, Debug)]
pub struct LinearEquation {
    m:    f64,
    c:    f64,
    root: Option<f64>
}

impl LinearEquation {
    pub fn new(m: f64, c: f64) -> LinearEquation {
        if m == 0.0
        { panic!("m cannot be zero."); }
        LinearEquation { m, c, root: None }
    }

    pub fn m(&self) -> f64 {
        self.m
    }

    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn set_m(&mut self, value: f64) {
        if value == 0.0
        { panic!("m cannot be zero."); }
        self.m = value
    }

    pub fn set_c(&mut self, value: f64) {
        self.c = value;
    }

    pub fn get_root(&mut self) -> f64 {
        if self.root.is_some()
        { return unsafe { self.root.unwrap_unchecked() }; }

        self.root = Some(- self.c / self.m);
        unsafe { self.root.unwrap_unchecked() }
    }

    pub fn get_fun_val_of<T>(&self, x: T) -> f64
    where f64: From<T> {
        self.m * f64::from(x) + self.c
    }
}

impl std::fmt::Display for LinearEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f(x) = {}x + {}", self.m, self.c)
    }
}
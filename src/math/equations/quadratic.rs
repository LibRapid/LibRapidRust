use crate::math::general::CommonPowers;
#[derive(Clone, Copy, Debug)]
pub struct QuadraticEquation {
    a:         f64,
    b:         f64,
    c:         f64,
    vertex:    Option<(f64, f64)>,
    solutions: Option<(f64, f64)>
}

impl QuadraticEquation {
    pub fn new() -> QuadraticEquation {
        QuadraticEquation { a:         1.0,
                            b:         0.0,
                            c:         0.0,
                            vertex:    Some((0.0, 0.0)),
                            solutions: Some((0.0, 0.0)) }
    }

    pub fn new_from_coefficients(a: f64, b: f64, c: f64) -> QuadraticEquation {
        if a == 0.0
        { panic!("a was zero and is thus not allowed."); }
        QuadraticEquation { a,
                            b,
                            c,
                            vertex:    None,
                            solutions: None }
    }
    
    pub fn a(&self) -> f64 {
        self.a
    }
    
    pub fn b(&self) -> f64 {
        self.b
    }
    
    pub fn c(&self) -> f64 {
        self.c 
    }
    
    pub fn set_a(&mut self, value: f64) {
        if value == 0.0
        { panic!("a was zero and is thus not allowed."); }
        self.a = value;
    }
    
    pub fn set_b(&mut self, value: f64) {
        self.b = value;
    }
    
    pub fn set_c(&mut self, value: f64) {
        self.c = value;
    }

    pub fn get_solutions(&mut self) -> Option<(f64, f64)> {
        if self.solutions.is_some()
        { return self.solutions; }
        let discriminant = self.b.square() - 4.0 * self.a * self.c;
        if discriminant < 0.0
        { return None; }
        let x_1 = (- self.b + discriminant.sqrt()) / (2.0 * self.a);
        let x_2 = (- self.b - discriminant.sqrt()) / (2.0 * self.a);
    
        self.solutions = Some((x_1, x_2));

        self.solutions
    }

    pub fn get_vertex(&mut self) -> (f64, f64) {
        if self.vertex.is_some()
        { return unsafe { self.vertex.unwrap_unchecked() }; }
        let x = - self. b / (2.0 * self.a);
        self.vertex = Some((x, self.a * x.square() + self.b * x + self.c));
        unsafe { self.vertex.unwrap_unchecked() }
    }

    pub fn get_fun_val_of<T>(&self, x: T) -> f64
    where f64: From<T> {
        let _x = f64::from(x);
        self.a * _x.square() + self.b * _x + self.c
    }
}

impl std::fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f(x) = {}x^2 + {}x + {}", self.a, self.b, self.c)
    }
}
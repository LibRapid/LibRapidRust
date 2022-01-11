use number_theory::arithmetic::mpz::Mpz;

#[derive(Clone)]
pub struct BigInt {
    num: Mpz,
}

impl BigInt {
    pub fn zero() -> Self {
        BigInt { num: Mpz::zero() }
    }

    pub fn one() -> Self {
        BigInt { num: Mpz::one() }
    }

    pub fn from_string(x: &str) -> Option<Self> {
        match Mpz::u_from_string(x) {
            Some(num) => Some(BigInt { num }),
            None => None,
        }
    }

    pub fn to_string(&self) -> String {
        self.num.to_string()
    }

    pub fn sqrt(&self) -> Self {
        BigInt {
            num: self.num.sqrt(),
        }
    }

    pub fn nth_root(&self, n: u64) -> Self {
        BigInt {
            num: self.num.nth_root(n),
        }
    }

    pub fn pow(&self, p: u64) -> Self {
        BigInt {
            num: self.num.pow(p),
        }
    }

    pub fn log(&self, log: f64) -> f64 {
        self.num.log(log)
    }

    pub fn factorial(n: u64) -> Self {
        BigInt {
            num: Mpz::sirp(1, n, 1, 0),
        }
    }

    pub fn is_prime(&self) -> bool {
        self.num.probable_prime()
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigInt {
            num: self.num.addition(other.num),
        }
    }
}

impl std::ops::Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        BigInt {
            num: self.num.subtraction(other.num),
        }
    }
}

impl std::ops::Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        BigInt {
            num: self.num.product(other.num),
        }
    }
}

impl std::ops::Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        BigInt {
            num: self.num.euclidean(&other.num).0,
        }
    }
}

impl std::ops::Rem for BigInt {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        BigInt {
            num: self.num.euclidean(&other.num).1,
        }
    }
}

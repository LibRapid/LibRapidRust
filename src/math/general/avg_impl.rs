use std::collections::HashMap;
use super::{Averages, Increment};

// Averages don't work properly when implemented using generics, I don't know why.

impl Averages<f64> for Vec<f64> {
    fn arithmetic_mean(&self) -> f64 {
        self.iter().sum::<f64>() / self.len() as f64
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by(i.recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.len() & 1 != 0
        { return cloned[self.len() >> 1usize]; }
        let fst = cloned[self.len() / 2 - 1];
        let snd = cloned[self.len() / 2];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> f64 {
        panic!("Error: Mode is not implemented yet for f64/f32.")
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());

        (cloned[0] + cloned[cloned.len() - 1]) / 2.0
    }
}

impl Averages<f32> for Vec<f32> {
    fn arithmetic_mean(&self) -> f64 {
        (self.iter().sum::<f32>() / self.len() as f32) as f64
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by(i.recip());
        }

        (self.len() as f32 / r) as f64
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> f32 {
        panic!("Error: Mode is not implemented yet for f64/f32.")
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}

impl Averages<i8> for Vec<i8> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> i8 {
        let mut hm: HashMap<i8, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}

impl Averages<u8> for Vec<u8> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> u8 {
        let mut hm: HashMap<u8, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<u16> for Vec<u16> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> u16 {
        let mut hm: HashMap<u16, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<i16> for Vec<i16> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> i16 {
        let mut hm: HashMap<i16, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<u32> for Vec<u32> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> u32 {
        let mut hm: HashMap<u32, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<i32> for Vec<i32> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<u64> for Vec<u64> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> u64 {
        let mut hm: HashMap<u64, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<i64> for Vec<i64> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> i64 {
        let mut hm: HashMap<i64, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<u128> for Vec<u128> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> u128 {
        let mut hm: HashMap<u128, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<i128> for Vec<i128> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> i128 {
        let mut hm: HashMap<i128, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<usize> for Vec<usize> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> usize {
        let mut hm: HashMap<usize, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}


impl Averages<isize> for Vec<isize> {
    fn arithmetic_mean(&self) -> f64 {
        let mut s = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f64 / (self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        let mut r = 0.0;
        for i in self {
            r.inc_by((*i as f64).recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f64; }
        let fst = cloned[self.len() / 2];
        let snd = cloned[self.len() / 2 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> isize {
        let mut hm: HashMap<isize, usize> = HashMap::new();
        for x in self {
            *hm.entry(*x).or_default() += 1;
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> f64 {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f64 / 2.0
    }
}

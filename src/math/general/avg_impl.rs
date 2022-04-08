use std::{collections::HashMap, intrinsics::transmute};

use super::{Averages, NumTools};

// Averages don't work properly when implemented using generics, I don't know why.

impl Averages<f64> for Vec<f64> {
    type Output = f64;

    fn arithmetic_mean(&self) -> Self::Output {
        self.iter().sum::<f64>() / self.len() as f64
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f64 = 0.0;
        for i in self {
            r.inc_by(i.recip());
        }

        self.len() as f64 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<f64> = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.len() & 1 != 0
        { return cloned[self.len() >> 1usize]; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f64) / 2.0
    }

    fn mode(&self) -> f64 {
        let mut hm: HashMap<u64, usize> = HashMap::new();
        for x in self {
            hm.entry(x.to_bits()).or_default().inc();
        }
        unsafe {
            transmute(*hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap())
        }
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<f64> = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());

        (cloned[0] + cloned[cloned.len() - 1]) / 2.0
    }
}

impl Averages<f32> for Vec<f32> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        self.iter().sum::<f32>() / self.len() as f32
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by(i.recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<f32> = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1]; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        (fst + snd) / 2.0
    }

    fn mode(&self) -> f32 {
        let mut hm: HashMap<u32, usize> = HashMap::new();
        for x in self {
            hm.entry(x.to_bits()).or_default().inc();
        }
        unsafe {
            transmute(*hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap())
        }
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<f32> = self.clone();
        cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());

        (cloned[0] + cloned[cloned.len() - 1]) / 2.0
    }
}

impl Averages<i8> for Vec<i8> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<i8> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> i8 {
        let mut hm: HashMap<i8, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}

impl Averages<u8> for Vec<u8> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<u8> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> u8 {
        let mut hm: HashMap<u8, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<u8> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<u16> for Vec<u16> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<u16> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> u16 {
        let mut hm: HashMap<u16, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<i16> for Vec<i16> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<i16> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> i16 {
        let mut hm: HashMap<i16, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<u32> for Vec<u32> {
    type Output = f32;
    
    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<u32> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> u32 {
        let mut hm: HashMap<u32, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<i32> for Vec<i32> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<i32> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<i32> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<u64> for Vec<u64> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<u64> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> u64 {
        let mut hm: HashMap<u64, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<u64> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<i64> for Vec<i64> {
    type Output = f32;
    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<i64> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> i64 {
        let mut hm: HashMap<i64, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<i64> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<u128> for Vec<u128> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<u128> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> u128 {
        let mut hm: HashMap<u128, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<u128> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<i128> for Vec<i128> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<i128> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> i128 {
        let mut hm: HashMap<i128, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<usize> for Vec<usize> {
    type Output = f32;

    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<usize> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> usize {
        let mut hm: HashMap<usize, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<usize> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}


impl Averages<isize> for Vec<isize> {
    type Output = f32;
    fn arithmetic_mean(&self) -> Self::Output {
        let mut s: usize = 0;
        for i in self {
            s.inc_by(*i as usize);
        }
        s as f32 / (self.len() as f32)
    }

    fn harmonic_mean(&self) -> Self::Output {
        let mut r: f32 = 0.0;
        for i in self {
            r.inc_by((*i as f32).recip());
        }

        self.len() as f32 / r
    }

    fn median(&self) -> Self::Output {
        let mut cloned: Vec<isize> = self.clone();
        cloned.sort_unstable();
        if self.len() & 1 == 0
        { return cloned[self.len() >> 1] as f32; }
        let fst = cloned[self.len() >> 1];
        let snd = cloned[self.len() >> 1 + 1];

        ((fst + snd) as f32) / 2.0
    }

    fn mode(&self) -> isize {
        let mut hm: HashMap<isize, usize> = HashMap::new();
        for x in self {
            hm.entry(*x).or_default().inc();
        }
        *hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap()
    }

    fn mid_range(&self) -> Self::Output {
        let mut cloned: Vec<isize> = self.clone();
        cloned.sort_unstable();

        (cloned[0] + cloned[cloned.len() - 1]) as f32 / 2.0
    }
}
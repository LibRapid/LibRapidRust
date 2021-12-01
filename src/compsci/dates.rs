#![allow(dead_code)]

pub struct Date {
    year:      i32,
    month:     u8,
    day:       u8,
    hour:      u8,
    minute:    u8,
    second:    u8,
    leap_year: bool,
}

impl Date {
    pub fn new_ymd(year: i32, month: u8, day: u8) -> Option<Date> {
        if is_valid_day(year, month, day) {
            return Option::Some(Date { year,
                                       month,
                                       day,
                                       hour:   0,
                                       minute: 0,
                                       second: 0,
                                       leap_year:
                                       is_leap_year(year) });
        }
        return None;
    }

    pub fn with_hms(&mut self, hour: u8, minute: u8, second: u8) {
        match is_valid_hms(hour, minute, second) {
            true  => { 
                       self.hour   = hour;
                       self.minute = minute;
                       self.second = second; }
            false => { core::panic!("Error: Expected valid hour, minute and second.") }
        }
    }
    
    pub fn leap_year(&self) -> bool {
        self.leap_year
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn set_year(&mut self, year: i32) {
        match is_valid_day(year, self.month, self.day) {
            true  => { self.leap_year = is_leap_year(year); }
            false => { core::panic!("Error: Invalid year.") }
        }
        self.year = year;
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn set_month(&mut self, month: u8) {
        match is_valid_day(self.year, month, self.day) {
            true  => { self.month = month; }
            false => { core::panic!("Error: Invalid month.") }
        }
    }


    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn set_day(&mut self, day: u8) {
        match is_valid_day(self.year, self.month, day) {
            true  => { self.day = day; }
            false => { core::panic!("Error: Invalid day.") }
        }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn set_hour(&mut self, hour: u8) {
        match is_valid_hms(hour, self.minute, self.second) {
            true  => { self.hour = hour; }
            false => { core::panic!("Error: Invalid hour.") }
        }
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn set_minute(&mut self, minute: u8) {
        match is_valid_hms(self.hour, minute, self.second) {
            true  => { self.minute = minute; }
            false => { core::panic!("Error: Invalid minute.") }
        }
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn set_second(&mut self, second: u8) {
        match is_valid_hms(self.hour, self.minute, second) {
            true  => { self.second = second; }
            false => { core::panic!("Error: Invalid second.") }
        }
    }
}

fn is_valid_day(year: i32, month: u8, day: u8) -> bool {
    let day_in_month: u8;

    match month {
        2  => { if is_leap_year(year)
                { day_in_month = 29; }
                else
                { day_in_month = 28; }
            }
        4  => day_in_month = 30,
        6  => day_in_month = 30,
        9  => day_in_month = 30,
        11 => day_in_month = 30,
        _  => day_in_month = 31,
    }
    if day <= day_in_month { return true; }
    return false;
}

pub fn is_leap_year(year: i32) -> bool {
    year & 3 == 0 && (year & 24 == 0 || year & 15 == 0)
}

fn is_valid_hms(hour: u8, minute: u8, second: u8) -> bool {
    hour <= 24 && minute <= 60 && second <= 60
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let finstring: String =  self.year.to_string()    + "-" +
             &format!("{:0>2}", &self.month.to_string())  + "-" +
             &format!("{:0>2}", &self.day.to_string())    + " " +
             &format!("{:0>2}", &self.hour.to_string())   + ":" +
             &format!("{:0>2}", &self.minute.to_string()) + ":" +
             &format!("{:0>2}", &self.second.to_string()) + ":" ;
        write!(f, "{}", finstring)
    }
}
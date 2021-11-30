#![allow(dead_code)]

use core::panic;

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
            false => { panic!("Error: Expected valid hour, minute or second.") }
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
            true  => { if is_leap_year(year)
                       { self.leap_year = true; }
                       else
                       { self.leap_year = false; }
                }
            false => { panic!("Error: Invalid year.") }
        }
        self.year = year;
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn set_month(&mut self, month: u8) {
        match is_valid_day(self.year, month, self.day) {
            true  => { self.month = month; }
            false => { panic!("Error: Invalid month.") }
        }
    }


    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn set_day(&mut self, day: u8) {
        match is_valid_day(self.year, self.month, day) {
            true  => { self.day = day; }
            false => { panic!("Error: Invalid day.") }
        }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn set_hour(&mut self, hour: u8) {
        match is_valid_hms(hour, self.minute, self.second) {
            true  => { self.hour = hour; }
            false => { panic!("Error: Invalid hour.") }
        }
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn set_minute(&mut self, minute: u8) {
        match is_valid_hms(self.hour, minute, self.second) {
            true  => { self.minute = minute; }
            false => { panic!("Error: Invalid minute.") }
        }
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn set_second(&mut self, second: u8) {
        match is_valid_hms(self.hour, self.minute, second) {
            true  => { self.second = second; }
            false => { panic!("Error: Invalid second.") }
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
        3  => day_in_month = 30,
        5  => day_in_month = 30,
        8  => day_in_month = 30,
        10 => day_in_month = 30,
        _  => day_in_month = 31,
    }
    if day_in_month <= day { return true; }
    return false;
}

pub fn is_leap_year(year: i32) -> bool {
    if year % 4   != 0 { return false; }
    if year % 100 == 0 {

        if year % 400 == 0 { return true; }
        else { return false; }
    }
    return true;
}

fn is_valid_hms(hour: u8, minute: u8, second: u8) -> bool {
    hour <= 24 && minute <= 60 && second <= 60
}
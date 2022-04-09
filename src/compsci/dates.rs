//! Simple Dates. Do not expect anything fancy from this part of the library, as the focus does not lie on date-functions.
use core::cmp::Ordering::Equal;
/// The structure for dates in LibRapid.
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
    /// Initializes a new date with the given parameters.
    ///
    /// # Arguments
    /// * `year` - The Year.
    /// * `month` - The month.
    /// * `day` - The day.
    ///
    /// # Returns
    /// `none` if the date is invalid, otherwise a new date.
    #[must_use]
    pub const fn new_ymd(year: i32, month: u8, day: u8) -> Option<Date> {
        if is_valid_day(year, month, day) {
            return Some(Date { year,
                               month,
                               day,
                               hour:   0,
                               minute: 0,
                               second: 0,
                               leap_year:
                               is_leap_year(year) });
        }
        None
    }
    /// Adds a given amount of hours to the initialised date.
    ///
    /// #Arguments
    /// * `hour` - The hour.
    /// * `minute` - The minute.
    /// * `second` - The second.
    ///
    /// # Returns
    /// Nothing.
    pub fn with_hms(&mut self, hour: u8, minute: u8, second: u8) {
        match is_valid_hms(hour, minute, second) {
            true  => { 
                       self.hour   = hour;
                       self.minute = minute;
                       self.second = second; }
            false => { panic!("Error: Expected valid hour, minute and second.") }
        }
    }
    /// Checks if a year is a leap year.
    ///
    /// # Returns
    /// A boolean value.
    #[must_use]
    pub const fn leap_year(&self) -> bool {
        self.leap_year
    }
    /// Gets the year.
    ///
    /// # Returns
    /// A `i32`.
    #[must_use]
    pub const fn year(&self) -> i32 {
        self.year
    }
    /// Sets the year.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_year(&mut self, year: i32) {
        match is_valid_day(year, self.month, self.day) {
            true  => { self.leap_year = is_leap_year(year); }
            false => { panic!("Error: Invalid year.") }
        }
        self.year = year;
    }
    /// Gets the month.
    ///
    /// # Returns
    /// A `&u8`.
    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }
    /// Sets the month.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_month(&mut self, month: u8) {
        match is_valid_day(self.year, month, self.day) {
            true  => { self.month = month; }
            false => { panic!("Error: Invalid month.") }
        }
    }
    /// Gets the day.
    ///
    /// # Returns
    /// A `u8`.
    pub const fn day(&self) -> u8 {
        self.day
    }
    /// Sets the day.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_day(&mut self, day: u8) {
        match is_valid_day(self.year, self.month, day) {
            true  => { self.day = day; }
            false => { panic!("Error: Invalid day.") }
        }
    }
    /// Gets the hour.
    ///
    /// # Returns
    /// A `u8`.
    #[must_use]
    pub const fn hour(&self) -> u8 {
        self.hour
    }
    /// Sets the hour.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_hour(&mut self, hour: u8) {
        match is_valid_hms(hour, self.minute, self.second) {
            true  => { self.hour = hour; }
            false => { panic!("Error: Invalid hour.") }
        }
    }
    /// Gets the minute.
    ///
    /// # Returns
    /// A `u8`.
    #[must_use]
    pub const fn minute(&self) -> u8 {
        self.minute
    }
    /// Sets the minute.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_minute(&mut self, minute: u8) {
        match is_valid_hms(self.hour, minute, self.second) {
            true  => { self.minute = minute; }
            false => { panic!("Error: Invalid minute.") }
        }
    }
    /// Gets the second.
    ///
    /// # Returns
    /// A `u8`.
    #[must_use]
    pub const fn second(&self) -> u8 {
        self.second
    }
    /// Sets the second.
    ///
    /// # Returns
    /// Nothing.
    pub fn set_second(&mut self, second: u8) {
        match is_valid_hms(self.hour, self.minute, second) {
            true  => { self.second = second; }
            false => { panic!("Error: Invalid second.") }
        }
    }
}

const fn is_valid_day(year: i32, month: u8, day: u8) -> bool {
    let day_in_month: u8;

    match month {
        2  => { if is_leap_year(year)
                { day_in_month = 29; }
                else
                { day_in_month = 28; }
            }
        4 | 6 | 9 | 11  => day_in_month = 30,
        _  => day_in_month = 31,
    }
    if day <= day_in_month { return true; }
    false
}
/// Checks if a year is a leap year.
///
/// # Arguments
/// * `year` - The year to be checked.
/// # Returns
/// A boolean value.
#[inline]
#[must_use]
pub const fn is_leap_year(year: i32) -> bool {
    year & 3 == 0 && (year % 25 == 0 || year & 15 == 0)
}
#[inline]
const fn is_valid_hms(hour: u8, minute: u8, second: u8) -> bool {
    hour <= 24 && minute <= 60 && second <= 60
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let finstring: String =  self.year.to_string()    + "-" +
             &format!("{:0>2}", &self.month.to_string())  + "-" +
             &format!("{:0>2}", &self.day.to_string())    + " " +
             &format!("{:0>2}", &self.hour.to_string())   + ":" +
             &format!("{:0>2}", &self.minute.to_string()) + ":" +
             &format!("{:0>2}", &self.second.to_string());
        write!(f, "{}", finstring)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year   == other.year   &&
        self.month  == other.month  &&
        self.day    == other.day    &&
        self.hour   == other.hour   &&
        self.minute == other.minute &&
        self.second == other.second
    }
}

impl core::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(Equal) => {}
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(Equal) => {}
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(Equal) => {}
            ord => return ord,
        }
        match self.hour.partial_cmp(&other.hour) {
            Some(Equal) => {}
            ord => return ord,
        }
        match self.minute.partial_cmp(&other.minute) {
            Some(Equal) => {}
            ord => return ord,
        }
        match self.second.partial_cmp(&other.second) {
            Some(Equal) => {}
            ord => return ord,
        }
        None
    }
}
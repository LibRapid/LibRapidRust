struct Date {
    year:   i32,
    month:  u8,
    day:    u8,
    hour:   u8,
    minute: u8,
    second: u8,
}

impl Date {
    fn new_ymd(year: i32, month: u8, day: u8) -> Option<Date> {
        if is_valid_day(year, month, day) {
            
        }
        return None;
    }

    fn new() {
        
    }
}

fn is_valid_day(year: i32, month: u8, day: u8) -> bool {
    let day_in_month: u8;

    match month {
        1  => { if is_leap_year(year) { day_in_month = 29; } else { day_in_month = 28; } }
        3  => day_in_month = 30,
        5  => day_in_month = 30,
        8  => day_in_month = 30,
        10 => day_in_month = 30,
        _  => day_in_month = 31,
    }
    if day_in_month <= day { return true; }
    return false;
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 != 0 { return false; }
    if year % 100 == 0 {
        if year % 400 == 0 { return true; }
        else { return false; }
    }
    return true;
}
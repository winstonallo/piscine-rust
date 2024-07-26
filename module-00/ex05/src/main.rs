// src/main.rs

fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("Year 0 is not valid.");
    }
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 5 | 7 | 10 | 12 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => panic!("Invalid month: {}", month),
    }
}

fn main() {
    let mut weekday = 1;
    let mut year = 1;
    let mut month = 1;

    while year <= 2024 {
        let days_in_month = num_days_in_month(year, month);

        if (weekday + 12) % 7 == 5 {
            println!("Friday, {} {}, {}", month_to_string(month), 13, year);
        }

        weekday = (weekday + days_in_month) % 7;
        if month == 12 {
            month = 1;
            year += 1;
        } else {
            month += 1;
        }
    }
}

fn month_to_string(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(1600));
        assert!(!is_leap_year(1500));
        assert!(is_leap_year(2004));
        assert!(!is_leap_year(2003));
    }

    #[test]
    fn test_num_days_in_month() {
        assert_eq!(num_days_in_month(2000, 2), 29);
        assert_eq!(num_days_in_month(2001, 2), 28);
        assert_eq!(num_days_in_month(2004, 2), 29);
        assert_eq!(num_days_in_month(2001, 1), 31);
        assert_eq!(num_days_in_month(2001, 4), 30);
        assert_eq!(num_days_in_month(2001, 6), 30);
        assert_eq!(num_days_in_month(2001, 9), 30);
        assert_eq!(num_days_in_month(2001, 11), 30);
    }

    #[test]
    #[should_panic]
    fn test_invalid_year() {
        is_leap_year(0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_month() {
        num_days_in_month(2000, 13);
    }
}

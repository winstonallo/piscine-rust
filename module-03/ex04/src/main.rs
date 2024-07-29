use std::fmt::Display;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeParseError::InvalidLength => write!(f, "error: invalid length"),
            TimeParseError::InvalidNumber => write!(f, "error: invalid number"),
            TimeParseError::MissingColon => write!(f, "error: missing ':'"),
        }
    }
}

impl std::str::FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        }

        let bytes = s.as_bytes();
        if !bytes[0].is_ascii_digit() || !bytes[1].is_ascii_digit() || !bytes[3].is_ascii_digit() || !bytes[4].is_ascii_digit() {
            return Err(TimeParseError::InvalidNumber);
        }

        if bytes[2] != b':' {
            return Err(TimeParseError::MissingColon);
        }

        let hours = ((bytes[0] - b'0') * 10 + (bytes[1] - b'0')) as u32;
        let minutes = ((bytes[3] - b'0') * 10 + (bytes[4] - b'0')) as u32;

        if hours > 23 || minutes > 59 {
            return Err(TimeParseError::InvalidNumber);
        }

        Ok(Self {
            hours,
            minutes,
        })
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} hours, {} minutes", self.hours, self.minutes)
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
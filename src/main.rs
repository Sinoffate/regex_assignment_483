use std::io;
use regex::Regex;
use lazy_static::lazy_static;
use phonenumber::{Mode, PhoneNumber};


fn main() {
    let ssn = get_ssn();
    let valid = validate_ssn(&ssn);
    if valid {
        println!("Valid SSN");
    } else {
        println!("Invalid SSN");
    }

    println!("Enter your phone number: ");
    let mut phone_input = String::new();
    io::stdin()
        .read_line(&mut phone_input)
        .expect("Failed to read line");

    let phone_input = phone_input.trim();
    match get_phone_number(phone_input) {
        Some(phone_number) => {
            println!("Valid phone number: {}", phone_number.format().mode(Mode::National).to_string());
        },
        None => {
            println!("Invalid phone number");
        }
    }

    println!("Enter your email: ");
    let mut email_input = String::new();
    io::stdin()
        .read_line(&mut email_input)
        .expect("Failed to read line");

    let email_input = email_input.trim();
    if validate_email(email_input) {
        println!("Valid email");
    } else {
        println!("Invalid email");
    }

    println!("Enter a class roster name (Last name, First name, MI):");
    let mut roster_name_input = String::new();
    io::stdin().read_line(&mut roster_name_input).expect("Failed to read input");

    let roster_name_input = roster_name_input.trim();
    if validate_name_roster(roster_name_input) {
        println!("The class roster name is valid.");
    } else {
        println!("The class roster name is invalid.");
    }

    println!("Enter a date (MM-DD-YYYY or MM/DD/YYYY):");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).expect("Failed to read input");

    let date_input = date_input.trim();
    if validate_date(date_input) {
        println!("The date is valid.");
    } else {
        println!("The date is invalid.");
    }
}

lazy_static! {
    static ref SSN_REGEX: Regex = Regex::new(r"^(?P<area>\d{3})[-\s]?(?P<group>\d{2})[-\s]?(?P<serial>\d{4})$").unwrap();
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new(r"^\s*\(?(\d{3})\)?[-\s]?(\d{3})[-\s]?(\d{4})\s*$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    static ref NAME_ROSTER_REGEX: Regex = Regex::new(r"^(?P<last>[a-zA-Z]+),\s*(?P<first>[a-zA-Z]+)(,\s*(?P<middle>[a-zA-Z]))*$").unwrap();
    static ref DATE_REGEX: Regex = Regex::new(r"^(?P<month>0[1-9]|1[0-2])[-/](?P<day>0[1-9]|[12]\d|3[01])[-/](?P<year>\d{4})$").unwrap();
}

fn get_ssn() -> String {
    let mut ssn = String::new();
    println!("Enter your SSN: ");
    io::stdin()
        .read_line(&mut ssn)
        .expect("Failed to read line");
    let ssn = ssn.trim();
    ssn.to_string()
}

fn validate_ssn(ssn: &str) -> bool {
    if let Some(captures) = SSN_REGEX.captures(ssn) {
        let area = captures.name("area").unwrap().as_str().parse::<u16>().unwrap();
        let group = captures.name("group").unwrap().as_str().parse::<u16>().unwrap();
        let serial = captures.name("serial").unwrap().as_str().parse::<u16>().unwrap();

        // Area, group, and serial numbers must not be 0.
        if area == 0 || group == 0 || serial == 0 {
            return false;
        }

        // Invalid area numbers (specific): 666 ; (range): 900-999
        if area == 666 || (area >= 900) {
            return false;
        }

        return true;
    }
    false
}

fn get_phone_number(phone: &str) -> Option<PhoneNumber> {
    if let Some(captures) = PHONE_NUMBER_REGEX.captures(phone) {
        let area_code = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let local_prefix = captures.get(2).unwrap().as_str().parse::<u16>().unwrap();
        let local_suffix = captures.get(3).unwrap().as_str().parse::<u16>().unwrap();

        let phone_number = format!("+1{}{}{}", area_code, local_prefix, local_suffix);
        return phonenumber::parse(Some("US".parse().unwrap()), &phone_number).ok();
    }
    None
}

fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

fn validate_name_roster(name_roster: &str) -> bool {
    NAME_ROSTER_REGEX.is_match(name_roster)
}

fn validate_date(date: &str) -> bool {
    if let Some(captures) = DATE_REGEX.captures(date) {
        let month = captures.name("month").unwrap().as_str().parse::<u16>().unwrap();
        let day = captures.name("day").unwrap().as_str().parse::<u16>().unwrap();
        let year = captures.name("year").unwrap().as_str().parse::<u16>().unwrap();

        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_leap_year(year) => 29,
            2 => 28,
            _ => return false,
        };

        if day > 0 && day <= days_in_month {
            return true;
        }
    }

    false
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}





// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ssn() {
        assert!(validate_ssn("001-01-0001"));
    }
    #[test]
    fn test_validate_ssn_with_dashes() {
        assert!(validate_ssn("167-18-0009"));
    }

    #[test]
    fn test_validate_ssn_with_spaces() {
        assert!(validate_ssn("123 01 6281"));
    }

    #[test]
    fn test_validate_ssn_with_dashes_and_spaces() {
        assert!(validate_ssn("724-34 8124"));
    }

    #[test]
    fn test_validate_ssn_with_spaces_and_dashes() {
        assert!(validate_ssn("123 45-6789"));
    }

    #[test]
    fn test_validate_ssn_with_invalid_area_number() {
        assert!(!validate_ssn("000-45-6789"));
    }

    #[test]
    fn test_validate_ssn_with_invalid_group_number() {
        assert!(!validate_ssn("123-00-6789"));
    }

    #[test]
    fn test_validate_ssn_with_invalid_serial_number() {
        assert!(!validate_ssn("123-45-0000"));
    }

    #[test]
    fn test_validate_ssn_with_invalid_area_number_666() {
        assert!(!validate_ssn("666-45-6789"));
    }

    #[test]
    fn test_validate_ssn_with_invalid_area_number_900() {
        assert!(!validate_ssn("900-45-6789"));
    }

    #[test]
    fn test_get_phone_number() {
        let phone_number = get_phone_number("2063311383");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_phone_number_with_dashes() {
        let phone_number = get_phone_number("509-331-1383");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_phone_number_with_spaces() {
        let phone_number = get_phone_number("206 301 1473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_phone_number_with_parentheses() {
        let phone_number = get_phone_number("(206)3011473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_phone_number_with_dashes_and_parentheses() {
        let phone_number = get_phone_number("(206) 301-1473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_get_phone_number() {
        let phone_number = get_phone_number("1233011473");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_get_phone_number_with_dashes() {
        let phone_number = get_phone_number("101-015-9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_get_phone_number_with_spaces() {
        let phone_number = get_phone_number("101 015 9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_get_phone_number_with_parentheses() {
        let phone_number = get_phone_number("(101)0159846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_get_phone_number_with_dashes_and_parentheses() {
        let phone_number = get_phone_number("(101) 015-9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

}

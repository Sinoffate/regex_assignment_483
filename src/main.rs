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
    match validate_phone_number(phone_input) {
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

    println!("Enter a house address (Street number, street name, and abbreviation):");
    let mut address_input = String::new();
    io::stdin().read_line(&mut address_input).expect("Failed to read input");

    let address_input = address_input.trim();
    if validate_address(address_input) {
        println!("The house address is valid.");
    } else {
        println!("The house address is invalid.");
    }

    println!("Enter a city, state abbreviation, and zip code (e.g. Seattle, WA 98101):");
    let mut city_state_zip_input = String::new();
    io::stdin().read_line(&mut city_state_zip_input).expect("Failed to read input");

    let city_state_zip_input = city_state_zip_input.trim();
    if validate_city_state_zip(city_state_zip_input) {
        println!("The city, state, and zip code are valid.");
    } else {
        println!("The city, state, and zip code are invalid.");
    }

    println!("Enter a military time without colons and with leading zeros for times under 10 (e.g. 0123):");
    let mut military_time_input = String::new();
    io::stdin().read_line(&mut military_time_input).expect("Failed to read input");

    let military_time_input = military_time_input.trim();
    if validate_military_time(military_time_input) {
        println!("The military time is valid.");
    } else {
        println!("The military time is invalid.");
    }

    println!("Enter a US currency amount down to the penny (e.g. $123,456,789.23):");
    let mut currency_input = String::new();
    io::stdin().read_line(&mut currency_input).expect("Failed to read input");

    let currency_input = currency_input.trim();
    if validate_currency(currency_input) {
        println!("The currency amount is valid.");
    } else {
        println!("The currency amount is invalid.");
    }

    println!("Enter a URL, optionally including http:// or https:// (e.g. https://www.example.com):");
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input).expect("Failed to read input");

    let url_input = url_input.trim();
    if validate_url(url_input) {
        println!("The URL is valid.");
    } else {
        println!("The URL is invalid.");
    }

    println!("Enter a password with at least 10 characters, including at least one upper case \
              character, one lower case character, one digit, one punctuation mark, and no more \
              than 3 consecutive lower case characters:");
    let mut password_input = String::new();
    io::stdin().read_line(&mut password_input).expect("Failed to read input");

    let password_input = password_input.trim();
    if validate_password(password_input) {
        println!("The password is valid.");
    } else {
        println!("The password is invalid.");
    }

    println!("Enter a text to find all words containing an odd number of alphabetic characters and ending in 'ion':");
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).expect("Failed to read input");

    let text_input = text_input.trim();
    let odd_ion_words = validate_odd_ion_words(text_input);
    if !odd_ion_words.is_empty() {
        println!("Odd 'ion' words found:");
        for word in odd_ion_words {
            println!("{}", word);
        }
    } else {
        println!("No odd 'ion' words found.");
    }
}

lazy_static! {
    static ref SSN_REGEX: Regex = Regex::new(r"^(?P<area>\d{3})[-\s]?(?P<group>\d{2})[-\s]?(?P<serial>\d{4})$").unwrap();
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new(r"^\s*\(?(\d{3})\)?[-\s]?(\d{3})[-\s]?(\d{4})\s*$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    static ref NAME_ROSTER_REGEX: Regex = Regex::new(r"^(?P<last>[a-zA-Z]+),\s*(?P<first>[a-zA-Z]+)(,\s*(?P<middle>[a-zA-Z]))*$").unwrap();
    static ref DATE_REGEX: Regex = Regex::new(r"^(?P<month>0[1-9]|1[0-2])[-/](?P<day>0[1-9]|[12]\d|3[01])[-/](?P<year>\d{4})$").unwrap();
    static ref ADDRESS_REGEX: Regex = Regex::new(r"^(?P<number>\d+)\s+(?P<street>[a-zA-Z\s]+)\s+(?P<type>(road|street|boulevard|avenue|r(?:d)?|st(?:r)?|blvd|ave))$").unwrap();
    static ref CITY_STATE_ZIP_REGEX: Regex = Regex::new(r"^(?P<city>[a-zA-Z\s]+),\s+(?P<state>AL|AK|AZ|AR|CA|CO|CT|DE|FL|GA|HI|ID|IL|IN|IA|KS|KY|LA|ME|MD|MA|MI|MN|MS|MO|MT|NE|NV|NH|NJ|NM|NY|NC|ND|OH|OK|OR|PA|RI|SC|SD|TN|TX|UT|VT|VA|WA|WV|WI|WY)\s+(?P<zip>\d{5}(-\d{4})?)$").unwrap();
    static ref MILITARY_TIME_REGEX: Regex = Regex::new(r"^([01]\d|2[0-3])([0-5]\d)$").unwrap();
    static ref CURRENCY_REGEX: Regex = Regex::new(r"^\$((\d{1,3}(,\d{3})*(\.\d{2})?)|(\d+(\.\d{2})?))$").unwrap();
    static ref URL_REGEX: Regex = Regex::new(r"^(?i)((http|https):\/\/)?[\w\-]+(\.[\w\-]+)+\.?(:\d+)?(\/\S*)?$").unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[!@#$%^&*()_+,.?\:{}|<>])(?i)(?:(?:(?=.*[a-z]{1,3})[a-zA-Z\d!@#$%^&*()_+,.?\:{}|<>]){10,})$").unwrap();
    static ref ODD_ION_WORDS_REGEX: Regex = Regex::new(r"\b(?:[a-zA-Z][a-zA-Z]*?){1}(?:[a-zA-Z][a-zA-Z]*?){1}ion\b").unwrap();
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

fn validate_phone_number(phone: &str) -> Option<PhoneNumber> {
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

fn validate_address(address: &str) -> bool {
    ADDRESS_REGEX.is_match(address)
}

fn validate_city_state_zip(input: &str) -> bool {
    CITY_STATE_ZIP_REGEX.is_match(input)
}

fn validate_military_time(time: &str) -> bool {
    MILITARY_TIME_REGEX.is_match(time)
}

fn validate_currency(amount: &str) -> bool {
    CURRENCY_REGEX.is_match(amount)
}

fn validate_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
}

fn validate_password(password: &str) -> bool {
    PASSWORD_REGEX.is_match(password)
}

fn validate_odd_ion_words(text: &str) -> Vec<String> {
    ODD_ION_WORDS_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect()
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
    fn test_validate_phone_number() {
        let phone_number = validate_phone_number("2063311383");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_validate_phone_number_with_dashes() {
        let phone_number = validate_phone_number("509-331-1383");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_validate_phone_number_with_spaces() {
        let phone_number = validate_phone_number("206 301 1473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_validate_phone_number_with_parentheses() {
        let phone_number = validate_phone_number("(206)3011473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_validate_phone_number_with_dashes_and_parentheses() {
        let phone_number = validate_phone_number("(206) 301-1473");
        match phone_number {
            Some(phone_number) => {
                assert!(phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_validate_phone_number() {
        let phone_number = validate_phone_number("1233011473");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_validate_phone_number_with_dashes() {
        let phone_number = validate_phone_number("101-015-9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_validate_phone_number_with_spaces() {
        let phone_number = validate_phone_number("101 015 9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_validate_phone_number_with_parentheses() {
        let phone_number = validate_phone_number("(101)0159846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_invalid_validate_phone_number_with_dashes_and_parentheses() {
        let phone_number = validate_phone_number("(101) 015-9846");
        match phone_number {
            Some(phone_number) => {
                assert!(!phone_number.is_valid());
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("notafed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_username() {
        assert!(validate_email("not-a-fed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_username_2() {
        assert!(validate_email("not_a_fed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_domain() {
        assert!(validate_email("definitelyNotAFed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_domain_2() {
        assert!(validate_email("definitelynotaFed@fed-fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_invalid_domain() {
        assert!(validate_email("notafed@f.b.i.gov"));
    }

    #[test]
    fn test_validate_email_with_invalid_domain_2() {
        assert!(validate_email("notafed@fbi..gov"));
    }

    #[test]
    fn test_validate_email_with_invalid_username() {
        assert!(validate_email("notafed-@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_invalid_username_2() {
        assert_eq!(validate_email("not...fed@domain.gov"), false);
    }

    #[test]
    fn test_validate_email_with_invalid_symbol() {
        assert_eq!(validate_email("notafbi.gov"), false);
    }

    #[test]
    fn test_validate_name_roster() {
        assert!(validate_name_roster("Doe, John, W"));
    }

    #[test]
    fn test_validate_name_roster_without_middle_initial() {
        assert!(validate_name_roster("Ded, Zed"));
    }

    #[test]
    fn test_validate_name_roster_with_multiple_middle_initial() {
        assert!(validate_name_roster("Roe, Joe, W, H"));
    }

    #[test]
    fn test_validate_name_roster_with_multiple_middle_initial_2() {
        assert!(validate_name_roster("Roe, Joe, W, H, J"));
    }

    #[test]
    fn test_validate_name_roster_with_multiple_middle_initial_3() {
        assert!(validate_name_roster("Roe, Joe, W, H, J, K"));
    }

    #[test]
    fn test_validate_name_roster_invalid() {
        assert_eq!(validate_name_roster("Roe Joe W H J K"), false);
    }

    #[test]
    fn test_validate_name_roster_invalid_2() {
        assert_eq!(validate_name_roster("Jingle Heimer Schmidt"), false);
    }


}

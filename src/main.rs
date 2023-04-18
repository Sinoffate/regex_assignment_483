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

}

lazy_static! {
    static ref SSN_REGEX: Regex = Regex::new(r"^(?P<area>\d{3})[-\s]?(?P<group>\d{2})[-\s]?(?P<serial>\d{4})$").unwrap();
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new(r"^\s*\(?(\d{3})\)?[-\s]?(\d{3})[-\s]?(\d{4})\s*$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    static ref NAME_ROSTER_REGEX: Regex = Regex::new(r"^(?P<last>[a-zA-Z]+),\s*(?P<first>[a-zA-Z]+)(,\s*(?P<middle>[a-zA-Z]))*$").unwrap();
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

        // Invalid area numbers (ranges): 734-749, 772-799, 800-999
        if (area >= 734 && area <= 749) || (area >= 772 && area <= 799) || (area >= 800) {
            return false;
        }

        // Invalid area numbers (specific values): 666
        if area == 666 {
            return false;
        }

        // Reserved for future use: 900-999
        if area >= 900 {
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

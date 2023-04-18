use std::io;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let ssn = get_ssn();
    let valid = validate_ssn(&ssn);
    if valid {
        println!("Valid SSN");
    } else {
        println!("Invalid SSN");
    }
}

lazy_static! {
    static ref SSN_REGEX: Regex = Regex::new(r"^(?P<area>\d{3})[-\s]?(?P<group>\d{2})[-\s]?(?P<serial>\d{4})$").unwrap();
}

fn validate_ssn(ssn: &str) -> bool {
    if let Some(captures) = SSN_REGEX.captures(ssn) {
        let area = captures.name("area").unwrap().as_str().parse::<u16>().unwrap();
        let group = captures.name("group").unwrap().as_str().parse::<u16>().unwrap();
        let serial = captures.name("serial").unwrap().as_str().parse::<u16>().unwrap();

        if area == 0 || group == 0 || serial == 0 {
            return false;
        }

        if area >= 900 {
            return false;
        }

        return true;
    }
    false
}

// get the user input for the SSN
fn get_ssn() -> String {
    let mut ssn = String::new();
    println!("Enter your SSN: ");
    io::stdin()
        .read_line(&mut ssn)
        .expect("Failed to read line");
    ssn
}
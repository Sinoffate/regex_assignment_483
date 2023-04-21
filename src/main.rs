use std::io;
use regex::Regex;
use phonenumber::{Mode, PhoneNumber};

fn main() {
    let ssn = get_ssn();
    let valid = validate_ssn(&ssn);
    if valid {
        println!("Valid SSN");
    } else {
        println!("Invalid SSN");
    }

    println!("\nEnter your phone number: ");
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

    println!("\nEnter your email: ");
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

    println!("\nEnter a class roster name (Last name, First name, MI):");
    let mut roster_name_input = String::new();
    io::stdin().read_line(&mut roster_name_input).expect("Failed to read input");

    let roster_name_input = roster_name_input.trim();
    if validate_name_roster(roster_name_input) {
        println!("The class roster name is valid.");
    } else {
        println!("The class roster name is invalid.");
    }

    println!("\nEnter a date (MM-DD-YYYY or MM/DD/YYYY):");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).expect("Failed to read input");

    let date_input = date_input.trim();
    if validate_date(date_input) {
        println!("The date is valid.");
    } else {
        println!("The date is invalid.");
    }

    println!("\nEnter a house address (Street number, street name, and abbreviation, e.g. 1234 Elm street):");
    let mut address_input = String::new();
    io::stdin().read_line(&mut address_input).expect("Failed to read input");

    let address_input = address_input.trim();
    if validate_address(address_input) {
        println!("The house address is valid.");
    } else {
        println!("The house address is invalid.");
    }

    println!("\nEnter a city, state abbreviation, and zip code (e.g. Seattle, WA 98101):");
    let mut city_state_zip_input = String::new();
    io::stdin().read_line(&mut city_state_zip_input).expect("Failed to read input");

    let city_state_zip_input = city_state_zip_input.trim();
    if validate_city_state_zip(city_state_zip_input) {
        println!("The city, state, and zip code are valid.");
    } else {
        println!("The city, state, and zip code are invalid.");
    }

    println!("\nEnter a military time without colons and with leading zeros for times under 10 (e.g. 0123):");
    let mut military_time_input = String::new();
    io::stdin().read_line(&mut military_time_input).expect("Failed to read input");

    let military_time_input = military_time_input.trim();
    if validate_military_time(military_time_input) {
        println!("The military time is valid.");
    } else {
        println!("The military time is invalid.");
    }

    println!("\nEnter a US currency amount down to the penny (e.g. $123,456,789.23):");
    let mut currency_input = String::new();
    io::stdin().read_line(&mut currency_input).expect("Failed to read input");

    let currency_input = currency_input.trim();
    if validate_currency(currency_input) {
        println!("The currency amount is valid.");
    } else {
        println!("The currency amount is invalid.");
    }

    println!("\nEnter a URL, optionally including http:// or https:// (e.g. https://www.example.com):");
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input).expect("Failed to read input");

    let url_input = url_input.trim();
    if validate_url(url_input) {
        println!("The URL is valid.");
    } else {
        println!("The URL is invalid.");
    }

    println!("\nEnter a password with at least 10 characters, including at least one upper case \
              character, one lower case character, \none digit, one punctuation mark, and no more \
              than 3 consecutive lower case characters:");
    let mut password_input = String::new();
    io::stdin().read_line(&mut password_input).expect("Failed to read input");

    let password_input = password_input.trim();
    if validate_password(password_input) {
        println!("The password is valid.");
    } else {
        println!("The password is invalid.");
    }

    println!("\nEnter a text to find all words containing an odd number of alphabetic characters and ending in 'ion':");
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).expect("Failed to read input");

    let text_input = text_input.trim();
    let odd_ion_words = validate_odd_ion_words(text_input);
    if !odd_ion_words.is_empty() {
        println!("Odd 'ion' words found:");
        for word in odd_ion_words {
            println!("{ }", word);
        }
    } else {
        println!("No odd 'ion' words found.");
    }
}

/// Retrieves a Social Security Number (SSN) from the user.
///
/// # Description
///
/// * Prompts the user to enter their SSN.
/// * Reads the user's input and trims any leading or trailing whitespace.
///
/// # Returns
///
/// * `String` - Returns the user's inputted SSN as a String.
fn get_ssn() -> String {
    let mut ssn = String::new();
    println!("Enter your SSN: ");

    // Read the user's input and store it in the 'ssn' variable.
    // If reading the input fails, an error message will be displayed.
    io::stdin()
        .read_line(&mut ssn)
        .expect("Failed to read line");

    // Trim any leading or trailing whitespace from the input and convert it to a String.
    let ssn = ssn.trim();
    ssn.to_string()
}

/// Validates a US Social Security Number (SSN).
///
/// # Arguments
///
/// * `ssn` - A string slice that holds the Social Security Number.
///
/// # Rules
///
/// * Accepts SSNs with or without dashes or spaces as separators.
/// * Area, group, and serial numbers must not be 0.
/// * Invalid area numbers: 666 and those in the range 900-999.
///
/// # Returns
///
/// * `bool` - Returns true if the SSN is valid, false otherwise.
fn validate_ssn(ssn: &str) -> bool {
    let ssn_regex: Regex = Regex::new(r"^(?P<area>\d{3})[-\s]?(?P<group>\d{2})[-\s]?(?P<serial>\d{4})$").unwrap();
    // Check if the SSN matches the SSN_REGEX pattern.
    // If it matches, the 'captures' variable will contain the matched components.
    if let Some(captures) = ssn_regex.captures(ssn) {
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

        // If the SSN passes all the validation checks, return true.
        return true;
    }

    // If the SSN does not match the SSN_REGEX pattern, return false.
    return false
}

/// Validates a US phone number.
///
/// # Arguments
///
/// * `phone` - A string slice that holds the phone number.
///
/// # Rules
///
/// * Accepts US phone numbers with or without parentheses around the area code.
/// * Accepts phone numbers with or without dashes or spaces as separators.
/// * Only allows valid area codes.
///
/// # Returns
///
/// * `Option<PhoneNumber>` - Returns Some(PhoneNumber) if the phone number is valid, None otherwise.
fn validate_phone_number(phone: &str) -> Option<PhoneNumber> {
    let phone_regex: Regex = Regex::new(r"^\s*\(?(\d{3})\)?[-\s]?(\d{3})[-\s]?(\d{4})\s*$").unwrap();
    // Check if the input phone number matches the PHONE_NUMBER_REGEX pattern.
    // If it matches, the 'captures' variable will contain the matched components.
    if let Some(captures) = phone_regex.captures(phone) {

        // Extract the 'area_code', 'local_prefix', and 'local_suffix' numbers
        // from the matched phone number components.
        let area_code = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let local_prefix = captures.get(2).unwrap().as_str().parse::<u16>().unwrap();
        let local_suffix = captures.get(3).unwrap().as_str().parse::<u16>().unwrap();

        // Format the phone number as "+1<area_code><local_prefix><local_suffix>"
        let phone_number = format!("+1{}{}{}", area_code, local_prefix, local_suffix);

        // Use the 'phonenumber' crate to parse and validate the formatted phone number.
        // If the phone number is valid, the function will return Some(PhoneNumber).
        // Otherwise, it will return None.
        return phonenumber::parse(Some("US".parse().unwrap()), &phone_number).ok();
    }

    // If the input phone number does not match the PHONE_NUMBER_REGEX pattern, the function returns None.
    None
}

/// Validates an email address according to the specified rules.
///
/// # Arguments
///
/// * `email` - A string slice that holds the email address to be validated.
///
/// # Rules
///
/// * Allowed special characters are !#$ %&'*/=? ^_+-`{|}~
/// * No consecutive special characters.
/// * Alphanumeric characters ignoring case.
/// * Only one @ symbol for separating the prefix and domain.
/// * Only a single . is allowed as a special character in the domain.
///
/// # Returns
///
/// * `bool` - Returns true if the email address is valid, false otherwise.
fn validate_email(email: &str) -> bool {
    let email_regex: Regex = Regex::new(r"(?i)^(?P<prefix>[a-z0-9!#$%&'*+/=?^_`{|}~-]+(\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*)(@)(?P<domain>[a-z0-9](?:[a-z0-9-]*[a-z0-9])?(\.[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)+)$").unwrap();
    email_regex.is_match(email)
}

/// Validates a name in the format of a class roster.
///
/// # Arguments
///
/// * `name_roster` - A string slice that holds the name in the format "Last, First, MI".
///
/// # Rules
///
/// * Last name, followed by a comma and a space.
/// * First name, followed by an optional comma and space, and middle initial(s).
/// * Each part of the name should only contain alphabetic characters.
///
/// # Returns
///
/// * `bool` - Returns true if the name is in the correct format, false otherwise.
fn validate_name_roster(name_roster: &str) -> bool {
    let name_roster_regex: Regex = Regex::new(r"^(?P<last>[a-zA-Z]+),\s*(?P<first>[a-zA-Z]+)(,\s*(?P<middle>[a-zA-Z]))*$").unwrap();
    name_roster_regex.is_match(name_roster)
}

/// Validates a given date string in the format "MM/DD/YYYY" or "MM-DD-YYYY".
///
/// # Description
///
/// * Checks if the date string matches the expected format using a regex pattern.
/// * Ensures the day value is valid for the given month and year.
/// * Accounts for leap years.
///
/// # Arguments
///
/// * `date: &str` - The input date string to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the date is valid, and `false` otherwise.
fn validate_date(date: &str) -> bool {
    let date_regex: Regex = Regex::new(r"^(?P<month>0[1-9]|1[0-2])[-/](?P<day>0[1-9]|[12]\d|3[01])[-/](?P<year>\d{4})$").unwrap();
    if let Some(captures) = date_regex.captures(date) {

        // Parses the month, day, and year values from the matched date string into u16 integers.
        let month = captures.name("month").unwrap().as_str().parse::<u16>().unwrap();
        let day = captures.name("day").unwrap().as_str().parse::<u16>().unwrap();
        let year = captures.name("year").unwrap().as_str().parse::<u16>().unwrap();

        // Determines the number of days in the given month.
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_leap_year(year) => 29,
            2 => 28,
            _ => return false,
        };

        // Checks if the day value is valid for the given month and year.
        if day > 0 && day <= days_in_month {
            return true;
        }
    }
    return false
}

/// Determines if a given year is a leap year.
///
/// # Description
///
/// * A leap year is divisible by 4, but not divisible by 100 unless it's also divisible by 400.
///
/// # Arguments
///
/// * `year: u16` - The input year to check for leap year status.
///
/// # Returns
///
/// * `bool` - Returns `true` if the year is a leap year, and `false` otherwise.
fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

/// Validates an address by checking if it matches the expected format.
///
/// # Description
///
/// * This function checks if the input address matches a specific format, which consists of:
///   - A house number (numeric)
///   - Street name (alphanumeric)
///   - Street type (e.g., road, street, avenue, etc.)
/// * The input address string should be in a case-insensitive format.
///
/// # Arguments
///
/// * `address: &str` - The input address string to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the address matches the expected format, and `false` otherwise.
fn validate_address(address: &str) -> bool {
    let address_regex: Regex = Regex::new(r"(?i)^\d+\s+([\w\s]+)\s+(road|rd|street|st|avenue|ave|boulevard|blvd)$").unwrap();
    address_regex.is_match(address)
}

/// Validates a city, state, and ZIP code combination by checking if it matches the expected format.
///
/// # Description
///
/// * This function checks if the input string matches the specific format, which consists of:
///   - City name (alphabetical characters and spaces)
///   - State abbreviation (2-letter US state abbreviation)
///   - ZIP code (5-digit ZIP code, optionally followed by a hyphen and 4 more digits)
/// * The input string should have a comma and a space separating the city and state, and a space separating the state and ZIP code.
///
/// # Arguments
///
/// * `input: &str` - The input string containing the city, state, and ZIP code to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the input matches the expected format, and `false` otherwise.
fn validate_city_state_zip(input: &str) -> bool {
    let city_state_zip_regex: Regex = Regex::new(r"^(?P<city>[a-zA-Z\s]+),\s+(?P<state>AL|AK|AZ|AR|CA|CO|CT|DE|FL|GA|HI|ID|IL|IN|IA|KS|KY|LA|ME|MD|MA|MI|MN|MS|MO|MT|NE|NV|NH|NJ|NM|NY|NC|ND|OH|OK|OR|PA|RI|SC|SD|TN|TX|UT|VT|VA|WA|WV|WI|WY)\s+(?P<zip>\d{5}(-\d{4})?)$").unwrap();
    city_state_zip_regex.is_match(input)
}

/// Validates a military time (24-hour) format string by checking if it matches the expected format.
///
/// # Description
///
/// * This function checks if the input string matches the military time format (24-hour format), which consists of:
///   - Hours: 00-23
///   - Minutes: 00-59
/// * The input string should be 4 digits long without any separator between hours and minutes.
///
/// # Arguments
///
/// * `time: &str` - The input string containing the military time to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the input matches the expected military time format, and `false` otherwise.
fn validate_military_time(time: &str) -> bool {
    let military_time_regex: Regex = Regex::new(r"^([01]\d|2[0-3])([0-5]\d)$").unwrap();
    military_time_regex.is_match(time)
}

/// Validates a currency amount string by checking if it matches the expected format.
///
/// # Description
///
/// * This function checks if the input string matches the currency amount format, which consists of:
///   - A dollar sign ($)
///   - An optional group of 1 to 3 digits followed by optional groups of 3 digits separated by commas
///   - An optional decimal point followed by exactly 2 digits
/// * The input string must start with a dollar sign ($).
///
/// # Arguments
///
/// * `amount: &str` - The input string containing the currency amount to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the input matches the expected currency amount format, and `false` otherwise.
fn validate_currency(amount: &str) -> bool {
    let currency_regex: Regex = Regex::new(r"^\$((\d{1,3}(,\d{3})*(\.\d{2})?)|(\d+(\.\d{2})?))$").unwrap();
    currency_regex.is_match(amount)
}

/// Validates a URL string by checking if it matches the expected format.
///
/// # Description
///
/// * This function checks if the input string matches the URL format, which consists of:
///   - An optional "http://" or "https://" prefix (case-insensitive)
///   - A domain name with one or more subdomains, each consisting of alphanumeric characters and hyphens, separated by periods
///   - A top-level domain (TLD) with 2 to 6 alphabetic characters
///   - An optional path with allowed characters: a-z, A-Z, 0-9, -, (, ), @, %, _, +, ., ~, #, ?, &, =
///
/// # Arguments
///
/// * `url: &str` - The input string containing the URL to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the input matches the expected URL format, and `false` otherwise.
fn validate_url(url: &str) -> bool {
    let url_regex: Regex = Regex::new(r"(?i)^(?:http[s]?://)?(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+(?:[a-zA-Z]{2,6})(?:/[-a-zA-Z0-9()@:%_+.~#?&=]*)?$").unwrap();
    url_regex.is_match(url)
}

/// Validates a password string by checking if it meets the specified requirements.
///
/// # Description
///
/// * This function checks if the input password string meets the following requirements:
///   - At least 10 characters in length
///   - Contains at least one uppercase character
///   - Contains at least one lowercase character
///   - Contains at least one digit
///   - Contains at least one punctuation mark
///   - Does not have more than 3 consecutive lowercase characters
///
/// # Arguments
///
/// * `password: &str` - The input string containing the password to validate.
///
/// # Returns
///
/// * `bool` - Returns `true` if the input password meets all the requirements, and `false` otherwise.
fn validate_password(password: &str) -> bool {

    // Create a regular expression for each requirement
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let digit = Regex::new(r"\d").unwrap();
    let punctuation = Regex::new(r"[:punct:]").unwrap();
    let consecutive_lowercase = Regex::new(r"[a-z]{4,}").unwrap();

    // Check if the password meets each requirement
    let min_length = password.len() >= 10;
    let has_uppercase = uppercase.is_match(password);
    let has_lowercase = lowercase.is_match(password);
    let has_digit = digit.is_match(password);
    let has_punctuation = punctuation.is_match(password);
    let no_consecutive_lowercase = !consecutive_lowercase.is_match(password);

    min_length && has_uppercase && has_lowercase && has_digit && has_punctuation && no_consecutive_lowercase
}

/// Validates a text string by extracting words that have an odd number of characters and end with "ion".
///
/// # Description
///
/// * This function checks the input text string for words that:
///   - Have an odd number of characters
///   - End with the substring "ion"
///   - The words can be delimited by spaces, newlines, tabs, or dashes.
///
/// * The function extracts all matching words and returns them as a vector of strings.
///
/// # Arguments
///
/// * `text: &str` - The input string containing the text to be validated and processed.
///
/// # Returns
///
/// * `Vec<String>` - A vector of strings containing the extracted words that meet the specified criteria.
fn validate_odd_ion_words(text: &str) -> Vec<String> {
    let odd_ion_words_regex: Regex = Regex::new(r"\b(?:[a-zA-Z]{2})*[a-zA-Z]ion(?:\b(?:\s|-|\n|\t)*|$)").unwrap();

    // Extract all words that match the criteria and return them as a vector of strings
    odd_ion_words_regex
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
    fn test_validate_email_with_valid_email() {
        assert!(validate_email("notafed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_username_dashes() {
        assert!(validate_email("not-a-fed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_username_underscores() {
        assert!(validate_email("not_a_fed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_domain() {
        assert!(validate_email("definitelyNotAFed@fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_valid_domain_dash() {
        assert!(validate_email("definitelynotaFed@fed-fbi.gov"));
    }

    #[test]
    fn test_validate_email_with_invalid_domain() {
        assert_eq!(validate_email("notafed@fb..i.gov"), false);
    }

    #[test]
    fn test_validate_email_with_invalid_domain_2() {
        assert_eq!(validate_email("notafed@fbi..gov"), false);
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

    #[test]
    fn test_validate_name_roster_invalid_with_dashes() {
        assert_eq!(validate_name_roster("Roe-Joe-W-H-J-K"), false);
    }

    #[test]
    fn test_validate_name_roster_invalid_with_underscores() {
        assert_eq!(validate_name_roster("Roe_Joe_W_H_J_K"), false);
    }

    #[test]
    fn test_validate_name_roster_invalid_with_semi_colons() {
        assert_eq!(validate_name_roster("Roe;Joe;W;H;J;K"), false);
    }

    #[test]
    fn validate_date_valid_with_dash_separator() {
        assert!(validate_date("12-31-2021"));
    }

    #[test]
    fn validate_date_valid_with_slash_separator() {
        assert!(validate_date("01/01/2022"));
    }

    #[test]
    fn validate_date_valid_leap_year() {
        assert!(validate_date("02-29-2020"));
    }

    #[test]
    fn validate_date_valid_month_boundary() {
        assert!(validate_date("03/31/2021"));
    }

    #[test]
    fn validate_date_valid_end_of_year() {
        assert!(validate_date("12-31-2023"));
    }

    #[test]
    fn validate_date_invalid_missing_separator() {
        assert!(!validate_date("03252021"));
    }

    #[test]
    fn validate_date_invalid_wrong_separator() {
        assert!(!validate_date("04.26.2021"));
    }

    #[test]
    fn validate_date_invalid_day_out_of_range() {
        assert!(!validate_date("02-30-2021"));
    }

    #[test]
    fn validate_date_invalid_month_out_of_range() {
        assert!(!validate_date("13/01/2021"));
    }

    #[test]
    fn validate_date_invalid_non_leap_year() {
        assert!(!validate_date("02-29-2021"));
    }

    #[test]
    fn validate_address_valid_full_road_name() {
        assert!(validate_address("1234 Elmwood Road"));
    }

    #[test]
    fn validate_address_valid_abbreviated_street_name() {
        assert!(validate_address("5678 Oak St"));
    }

    #[test]
    fn validate_address_valid_abbreviated_avenue_name() {
        assert!(validate_address("9012 Maple Ave"));
    }

    #[test]
    fn validate_address_valid_full_boulevard_name() {
        assert!(validate_address("3456 Cherry Boulevard"));
    }

    #[test]
    fn validate_address_valid_abbreviated_boulevard_name() {
        assert!(validate_address("7890 Pine Blvd"));
    }

    #[test]
    fn validate_address_invalid_no_street_number() {
        assert!(!validate_address("Cedar Road"));
    }

    #[test]
    fn validate_address_invalid_no_road_type() {
        assert!(!validate_address("1234 Walnut"));
    }

    #[test]
    fn validate_address_invalid_incorrect_road_abbreviation() {
        assert!(!validate_address("5678 Elm R"));
    }

    #[test]
    fn validate_address_invalid_missing_space() {
        assert!(!validate_address("9012Birch Street"));
    }

    #[test]
    fn validate_address_invalid_street_name_missing_space() {
        assert!(!validate_address("3456 PeachAve"));
    }

    #[test]
    fn validate_city_state_zip_valid_city_name() {
        assert!(validate_city_state_zip("Seattle, WA 98101"));
    }

    #[test]
    fn validate_city_state_zip_valid_state_abbreviation() {
        assert!(validate_city_state_zip("Austin, TX 78701"));
    }

    #[test]
    fn validate_city_state_zip_valid_zip_code() {
        assert!(validate_city_state_zip("New York, NY 10001"));
    }

    #[test]
    fn validate_city_state_zip_valid_with_spaces() {
        assert!(validate_city_state_zip("Los Angeles,   CA    90001"));
    }

    #[test]
    fn validate_city_state_zip_valid_multiline() {
        assert!(validate_city_state_zip("Portland,\nOR 97201"));
    }

    #[test]
    fn validate_city_state_zip_invalid_no_comma() {
        assert!(!validate_city_state_zip("Boston MA 02101"));
    }

    #[test]
    fn validate_city_state_zip_invalid_no_space_after_comma() {
        assert!(!validate_city_state_zip("Chicago,IL 60601"));
    }

    #[test]
    fn validate_city_state_zip_invalid_wrong_state_abbreviation() {
        assert!(!validate_city_state_zip("Miami, FLA 33101"));
    }

    #[test]
    fn validate_city_state_zip_invalid_non_alphabetic_city_name() {
        assert!(!validate_city_state_zip("123City, CA 90001"));
    }

    #[test]
    fn validate_city_state_zip_invalid_zip_code_length() {
        assert!(!validate_city_state_zip("Denver, CO 8020"));
    }

    #[test]
    fn validate_military_time_valid_midnight() {
        assert!(validate_military_time("0000"));
    }

    #[test]
    fn validate_military_time_valid_noon() {
        assert!(validate_military_time("1200"));
    }

    #[test]
    fn validate_military_time_valid_one_minute_before_midnight() {
        assert!(validate_military_time("2359"));
    }

    #[test]
    fn validate_military_time_valid_random_hour_and_minute() {
        assert!(validate_military_time("1543"));
    }

    #[test]
    fn validate_military_time_valid_leading_zero() {
        assert!(validate_military_time("0832"));
    }

    #[test]
    fn validate_military_time_invalid_too_short() {
        assert!(!validate_military_time("230"));
    }

    #[test]
    fn validate_military_time_invalid_too_long() {
        assert!(!validate_military_time("13452"));
    }

    #[test]
    fn validate_military_time_invalid_hour_out_of_range() {
        assert!(!validate_military_time("2500"));
    }

    #[test]
    fn validate_military_time_invalid_minute_out_of_range() {
        assert!(!validate_military_time("2370"));
    }

    #[test]
    fn validate_military_time_invalid_non_numeric() {
        assert!(!validate_military_time("abcd"));
    }

    #[test]
    fn validate_currency_valid_no_cents() {
        assert!(validate_currency("$1000"));
    }

    #[test]
    fn validate_currency_valid_with_cents() {
        assert!(validate_currency("$1234.56"));
    }

    #[test]
    fn validate_currency_valid_with_commas() {
        assert!(validate_currency("$1,234,567.89"));
    }

    #[test]
    fn validate_currency_valid_one_cent() {
        assert!(validate_currency("$0.01"));
    }

    #[test]
    fn validate_currency_valid_no_decimal_cents() {
        assert!(validate_currency("$1000.00"));
    }

    #[test]
    fn validate_currency_invalid_missing_dollar_sign() {
        assert!(!validate_currency("1234.56"));
    }

    #[test]
    fn validate_currency_invalid_wrong_decimal_places() {
        assert!(!validate_currency("$1234.567"));
    }

    #[test]
    fn validate_currency_invalid_non_numeric() {
        assert!(!validate_currency("$1,234.5a"));
    }

    #[test]
    fn validate_currency_invalid_comma_position() {
        assert!(!validate_currency("$12,34.56"));
    }

    #[test]
    fn validate_currency_invalid_extra_dollar_sign() {
        assert!(!validate_currency("$1$234.56"));
    }

    #[test]
    fn validate_url_valid_http() {
        assert!(validate_url("http://www.example.com"));
    }

    #[test]
    fn validate_url_valid_https() {
        assert!(validate_url("https://www.example.com"));
    }

    #[test]
    fn validate_url_valid_no_protocol() {
        assert!(validate_url("www.example.com"));
    }

    #[test]
    fn validate_url_valid_subdomain() {
        assert!(validate_url("https://subdomain.example.com"));
    }

    #[test]
    fn validate_url_valid_path_and_query() {
        assert!(validate_url("https://www.example.com/path?query=value"));
    }

    #[test]
    fn validate_url_invalid_missing_tld() {
        assert!(!validate_url("http://www.example"));
    }

    #[test]
    fn validate_url_invalid_space_in_url() {
        assert!(!validate_url("https://www.exa mple.com"));
    }

    #[test]
    fn validate_url_invalid_double_slash() {
        assert!(!validate_url("https://www.example.com//path"));
    }

    #[test]
    fn validate_url_invalid_extra_dot() {
        assert!(!validate_url("https://www..example.com"));
    }

    #[test]
    fn validate_url_invalid_wrong_protocol() {
        assert!(!validate_url("ftp://www.example.com"));
    }

    #[test]
    fn validate_password_valid_complex() {
        assert!(validate_password("A1b@c$d2E#"));
    }

    #[test]
    fn validate_password_valid_minimum_length() {
        assert!(validate_password("A1b#c2D!c1"));
    }

    #[test]
    fn validate_password_valid_no_consecutive_lowercase() {
        assert!(validate_password("A1b#cD2e#F3"));
    }

    #[test]
    fn validate_password_valid_all_requirements_met() {
        assert!(validate_password("A1b@c#D2e!F3"));
    }

    #[test]
    fn validate_password_valid_with_spaces() {
        assert!(validate_password("A1b @c #D2"));
    }

    #[test]
    fn validate_password_invalid_too_short() {
        assert!(!validate_password("A1b#c2"));
    }

    #[test]
    fn validate_password_invalid_missing_uppercase() {
        assert!(!validate_password("a1b#c2d$3"));
    }

    #[test]
    fn validate_password_invalid_missing_lowercase() {
        assert!(!validate_password("A1B@C#D2E$"));
    }

    #[test]
    fn validate_password_invalid_missing_digit() {
        assert!(!validate_password("AaBbCcDdEe"));
    }

    #[test]
    fn validate_password_invalid_consecutive_lowercase() {
        assert!(!validate_password("A1#@bcdefE"));
    }

    #[test]
    fn validate_odd_ion_words_valid_ablation() {
        assert_eq!(validate_odd_ion_words("ablation"), vec!["ablation"]);
    }

    #[test]
    fn validate_odd_ion_words_valid_ligation() {
        assert_eq!(validate_odd_ion_words("ligation"), vec!["ligation"]);
    }

    #[test]
    fn validate_odd_ion_words_valid_option() {
        assert_eq!(validate_odd_ion_words("option"), vec!["option"]);
    }

    #[test]
    fn validate_odd_ion_words_valid_fusion() {
        assert_eq!(validate_odd_ion_words("fusion"), vec!["fusion"]);
    }

    #[test]
    fn validate_odd_ion_words_valid_cation() {
        assert_eq!(validate_odd_ion_words("cation"), vec!["cation"]);
    }

    #[test]
    fn validate_odd_ion_words_invalid_even_chars() {
        assert_eq!(validate_odd_ion_words("differentiation"), Vec::<String>::new());
    }

    #[test]
    fn validate_odd_ion_words_invalid_not_ion() {
        assert_eq!(validate_odd_ion_words("evolutione"), Vec::<String>::new());
    }

    #[test]
    fn validate_odd_ion_words_invalid_single_char() {
        assert_eq!(validate_odd_ion_words("i"), Vec::<String>::new());
    }

    #[test]
    fn validate_odd_ion_words_invalid_no_chars() {
        assert_eq!(validate_odd_ion_words(""), Vec::<String>::new());
    }

    #[test]
    fn validate_odd_ion_words_invalid_empty_string() {
        assert_eq!(validate_odd_ion_words(" "), Vec::<String>::new());
    }
}

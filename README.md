[![Rust](https://github.com/Sinoffate/regex_assignment_483/actions/workflows/rust.yml/badge.svg)](https://github.com/Sinoffate/regex_assignment_483/actions/workflows/rust.yml)

# Rust Validation Project

This project demonstrates the usage of user input and regular expressions in Rust. It includes various functions for validating and parsing different types of inputs such as Social Security Numbers, US phone numbers, email addresses, and more.

## Prerequisites

To get started, you'll need to have the following software installed on your computer:

- Rust: Follow the [official installation guide](https://www.rust-lang.org/tools/install) to install Rust and its package manager, Cargo.

The installation process will install the Rust compiler, Cargo, and the Rust standard library. The dependencies will differ depending on your operating system.

For Windows, you can use the [Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to run the project on a Linux environment (Which will be the easier option than running natively).

## Getting Started
These instructions will help you set up the environment and run the project on your local machine.


  ```bash
  # Navigate to the project root directory   
  # To build the project we use Cargo:
    cargo build
  
  # then run the project with:
    cargo run

  # Running Tests
  # To run the unit tests for the validation functions, use the following command:
    cargo test

  # This command will run all the tests defined in the tests module and outputting the results and time in seconds to complete the tests.
  ```

## Functions
This project includes the following functions for validation and parsing:

- validate_ssn: Validates US Social Security Numbers.
- validate_phone_number: Validates US phone numbers and formats them in E.164 format.
- validate_email: Validates email addresses.
- validate_name: Validates names on a class roster.
- validate_date: Validates dates in MM-DD-YYYY format.
- validate_address: Validates US house addresses.
- validate_city_state_zip: Validates the city, state, and zip code format for a letter.
- validate_military_time: Validates military time without colons.
- validate_currency: Validates US currency amounts down to the penny.
- validate_url: Validates URLs with optional "http://" or "https://".
- validate_password: Validates passwords with specific requirements.
- validate_odd_ion_words: Validates words containing an odd number of alphabetic characters and ending in "ion".
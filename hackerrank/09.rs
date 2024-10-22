use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // Extract the AM/PM indicator
    let period = &s[8..10];
    // Extract hour, minute, and second components
    let hour: u32 = s[0..2].parse().unwrap();
    let minute = &s[3..5];
    let second = &s[6..8];

    // Convert hour based on AM/PM
    let converted_hour = match period {
        "AM" => {
            if hour == 12 {
                "00".to_string() // Midnight case (12 AM)
            } else {
                format!("{:02}", hour) // Keep the hour as is, formatted to two digits
            }
        }
        "PM" => {
            if hour == 12 {
                "12".to_string() // Noon case (12 PM)
            } else {
                format!("{:02}", hour + 12) // Convert PM hour to 24-hour format
            }
        }
        _ => unreachable!(), // This case should never happen with valid input
    };

    // Return the formatted time string in 24-hour format
    format!("{}:{}:{}", converted_hour, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

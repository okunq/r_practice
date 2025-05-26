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
let hour = &s[0..2];
    let minutes_seconds = &s[2..8];
    let period = &s[8..];

    let mut hour_num: u32 = hour.parse().unwrap();

    if period == "AM" {
        if hour_num == 12 {
            hour_num = 0;
        }
    } else if period == "PM" {
        if hour_num != 12 {
            hour_num += 12;
        }
    }

    format!("{:02}{}", hour_num, minutes_seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
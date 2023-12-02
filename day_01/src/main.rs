use std::fs;

use fancy_regex::Regex;

fn part1(input_str: &str) -> u32 {
    let mut sum = 0;
    for line in input_str.lines() {
        let mut start = 0;
        let mut end = 0;
        for char in line.chars() {
            match char.to_digit(10) {
                Some(x) => {
                    start = x;
                    break;
                }
                None => continue,
            }
        }
        for char in line.chars().rev() {
            match char.to_digit(10) {
                Some(x) => {
                    end = x;
                    break;
                }
                None => continue,
            }
        }
        sum += start * 10 + end;
    }
    sum
}

fn parse_digit(digit_str: &str) -> u32 {
    // print!("{digit_str}, ");
    match digit_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse().expect("Found an invalid value"),
    }
}

// This code is cleaner but much slower
fn part2_regex(input_str: &str) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"^.*?(?=([1-9]|one|two|three|four|five|six|seven|eight|nine)).*([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in input_str.lines() {
        let captures = re
            .captures(line)
            .expect("Error running regex")
            .expect("No match found");

        let start = captures
            .get(1)
            .expect("Didn't find a first capture group")
            .as_str();
        let end = captures
            .get(2)
            .expect("Didn't find a second capture group")
            .as_str();

        sum += parse_digit(start) * 10 + parse_digit(end);
    }
    sum
}

fn find_digit(test_str: &str) -> Option<u32> {
    let len = test_str.len();
    if len < 3 {
        return None;
    };
    match &test_str[len - 3..len] {
        "one" => {
            return Some(1);
        }
        "two" => {
            return Some(2);
        }
        "six" => {
            return Some(6);
        }
        _ => {}
    }
    match &test_str[0..3] {
        "one" => {
            return Some(1);
        }
        "two" => {
            return Some(2);
        }
        "six" => {
            return Some(6);
        }
        _ => {}
    }
    if len < 4 {
        return None;
    };
    match &test_str[len - 4..len] {
        "four" => {
            return Some(4);
        }
        "five" => {
            return Some(5);
        }
        "nine" => {
            return Some(9);
        }
        _ => {}
    }
    match &test_str[0..4] {
        "four" => {
            return Some(4);
        }
        "five" => {
            return Some(5);
        }
        "nine" => {
            return Some(9);
        }
        _ => {}
    }
    if len < 5 {
        return None;
    };
    match &test_str[len - 5..len] {
        "three" => {
            return Some(3);
        }
        "seven" => {
            return Some(7);
        }
        "eight" => {
            return Some(8);
        }
        _ => {}
    }
    match &test_str[0..5] {
        "three" => {
            return Some(3);
        }
        "seven" => {
            return Some(7);
        }
        "eight" => {
            return Some(8);
        }
        _ => {}
    }
    None
}

fn part2(input_str: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input_str.lines() {
        let len = line.len();
        for idx in 0..len {
            let test_str = line[..idx + 1].trim();

            if let Ok(x) = test_str[idx..idx + 1].parse::<u32>() {
                // print!("{x}");
                sum += x * 10;
                break;
            }

            if let Some(x) = find_digit(test_str) {
                // print!("{x}");
                sum += x * 10;
                break;
            }
        }
        for idx in 0..len {
            let test_str = line[(len - idx - 1)..].trim();

            if let Ok(x) = test_str[0..1].parse::<u32>() {
                // print!("{x}");
                sum += x;
                break;
            }

            if let Some(x) = find_digit(test_str) {
                // print!("{x}");
                sum += x;
                break;
            }
        }
        // println!();
    }
    sum
}

fn main() {
    let file_path = "input.txt";

    let input_str = fs::read_to_string(file_path).expect("Should have been able to read the file");

    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..1 {
        let p1 = part1(&input_str);
        println!("{p1}");
    }
    let elapsed = now.elapsed();
    println!("Elapsed for part1: {:.2?}", elapsed);

    let now = Instant::now();
    for _ in 0..1 {
        let p2 = part2(&input_str);
        println!("{p2}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed for part2: {:.2?}", elapsed);

    let now = Instant::now();
    for _ in 0..1 {
        let p2 = part2_regex(&input_str);
        println!("{p2}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed for part2_regex: {:.2?}", elapsed);
}

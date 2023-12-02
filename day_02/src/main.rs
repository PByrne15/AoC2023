use core::panic;
use std::{cmp::max, fs};

fn solution(input_str: &str) -> (u32, u32) {
    let mut s1: u32 = 0;
    let mut s2: u32 = 0;
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;
    for (idx, line) in input_str.lines().enumerate() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let (_, rounds) = line.split_once(':').expect("Didn't find expected ':");
        for round in rounds.split(';') {
            for pull in round.split(',') {
                let (count, colour) = pull
                    .trim()
                    .split_once(' ')
                    .expect("Didn't find expected whitespace");
                let count = count.parse().expect("Expected integer");
                match colour {
                    "red" => max_red = max(max_red, count),
                    "green" => max_green = max(max_green, count),
                    "blue" => max_blue = max(max_blue, count),
                    _ => panic!(""),
                }
            }
        }

        let adder: u32 = if RED >= max_red && GREEN >= max_green && BLUE >= max_blue {
            (idx + 1) as u32
        } else {
            0
        };

        s1 += adder;

        s2 += max_red * max_green * max_blue;
    }
    (s1, s2)
}

fn main() {
    let file_path = "input.txt";

    let input_str = fs::read_to_string(file_path).expect("Should have been able to read the file");

    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..1 {
        let (p1, p2) = solution(&input_str);
        println!("{p1}");
        println!("{p2}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

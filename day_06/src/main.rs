use std::{fs, iter::zip};

fn solution(input_str: &str) -> (i32, i64) {
    let mut lines = input_str.lines();
    let times_vec: Vec<i32> = lines
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .split(' ')
        .flat_map(|s| s.parse())
        .collect();
    let dist_vec: Vec<i32> = lines
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .split(' ')
        .flat_map(|s| s.parse())
        .collect();
    let races: Vec<(i32, i32)> = zip(times_vec, dist_vec).collect();

    let mut p1out = 1;
    for race in races {
        let (time, dist) = race;
        let mut winners = 0;
        for t in 0..time {
            if t * (time - t) > dist {
                winners += 1;
            }
        }

        if winners > 0 {
            p1out *= winners;
        }
    }

    lines = input_str.lines();
    let time: f64 = lines
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("");
    let dist: f64 = lines
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("");

    let v1 = ((time + f64::sqrt(time * time - (4.0 * dist))) / 2.0).floor() as i64;
    let v2 = ((time - f64::sqrt(time * time - (4.0 * dist))) / 2.0).ceil() as i64;

    let p2out = v1 - v2 + 1; // +1 because all the values between v2 and v1 are winners, inclusive of both

    (p1out, p2out)
}

fn main() {
    let file_path = "input.txt";

    let input_str = fs::read_to_string(file_path).expect("Should have been able to read the file");

    use std::time::Instant;
    let now = Instant::now();
    let n = 1;
    for _ in 0..n {
        let (p1, p2) = solution(&input_str);
        println!("{p1}");
        println!("{p2}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed for {} iterations: {:.2?}", n, elapsed);
}

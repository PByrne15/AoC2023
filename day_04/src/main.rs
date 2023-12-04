use std::{collections::HashSet, fs};

fn solution(input_str: &str) -> (i32, i32) {
    let mut sump1 = 0;
    let mut copy_vec = vec![1; input_str.lines().count()];
    for (idx, line) in input_str.lines().enumerate() {
        let (winning_numbers, our_numbers) =
            line.split_once(':').expect("").1.split_once('|').expect("");
        let winning_set: HashSet<i32> =
            HashSet::from_iter(winning_numbers.trim().split(' ').flat_map(|x| x.parse()));
        let our_set: HashSet<i32> =
            HashSet::from_iter(our_numbers.trim().split(' ').flat_map(|x| x.parse()));

        let matching_numbers = winning_set.intersection(&our_set).count();

        if matching_numbers > 0 {
            sump1 += 2_i32.pow(matching_numbers as u32 - 1);
            let copies = copy_vec[idx];
            for i in 1..=matching_numbers {
                copy_vec[idx + i] += copies;
            }
        }
    }

    let sump2: i32 = copy_vec.iter().sum();
    (sump1, sump2)
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

use std::{collections::HashSet, fs, vec};

fn solution(input_str: &str) -> (i32, i32) {
    let height = input_str.lines().count();
    let width = input_str
        .lines()
        .next()
        .expect("String has no lines")
        .chars()
        .count();

    let mut schem_vector: Vec<i32> = vec![-1; height * width];
    let mut num_index = 0;
    let mut num_vector: Vec<i32> = vec![];

    for (y, line) in input_str.lines().enumerate() {
        let mut curr_num = String::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    curr_num.push(c);
                    schem_vector[y * width + x] = num_index;
                }
                _ => {
                    if !curr_num.is_empty() {
                        num_vector.push(curr_num.parse::<i32>().expect(""));
                        num_index += 1;
                        curr_num = String::new();
                    }
                }
            }
        }
        if !curr_num.is_empty() {
            num_vector.push(curr_num.parse::<i32>().expect(""));
            num_index += 1;
        }
    }

    let mut indices_to_add: HashSet<i32> = HashSet::new();
    let mut sump2 = 0;

    for (y, line) in input_str.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' | '.' => {
                    continue;
                }
                symbol => {
                    let mut new_indices = HashSet::new();
                    for i in -1..=1 {
                        let yp = y as i32 + i;
                        if yp >= height as i32 || yp < 0 {
                            continue;
                        }
                        for j in -1..=1 {
                            let xp = x as i32 + j;
                            if xp >= width as i32 || xp < 0 {
                                continue;
                            }
                            let v = schem_vector[(yp * width as i32 + xp) as usize];
                            if v >= 0 {
                                new_indices.insert(v);
                            }
                        }
                    }
                    if symbol == '*' && new_indices.len() == 2 {
                        let values = new_indices.iter().map(|&i| num_vector[i as usize]);
                        sump2 += values.product::<i32>();
                    }

                    indices_to_add.extend(new_indices);
                }
            }
        }
    }

    let sump1: i32 = indices_to_add.iter().map(|&i| num_vector[i as usize]).sum();

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

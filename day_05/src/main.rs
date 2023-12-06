use std::{cmp::min, collections::HashMap, fs, ops::Range};

#[derive(PartialEq, Eq, Debug, Clone)]
struct StoDMap {
    source: i64,
    dest: i64,
    length: i64,
}

impl StoDMap {
    // Squashes the passed map into the existing map
    // Returns a new map covering just the overlapping sections
    fn squash(&self, other: &StoDMap) -> StoDMap {
        assert!(self.overlaps(other));

        // Overlaps at the start of the original map
        if other.source < self.dest && other.source + other.length > self.dest {
            let new_dest = other.dest + (self.dest - other.source);
            let new_length = min(other.length - (self.dest - other.source), self.length);
            return StoDMap {
                source: self.source,
                dest: new_dest,
                length: new_length,
            };
        }

        let new_source = self.source + (other.source - self.dest);
        let new_length = min(other.length, self.length - (other.source - self.dest));

        StoDMap {
            source: new_source,
            dest: other.dest,
            length: new_length,
        }
    }

    // Squashes the passed maps into the existing map
    // Returns a vector of maps covering the same source values with updated destinations
    fn squash_many(&self, others: &Vec<StoDMap>) -> Vec<StoDMap> {
        let mut overlaps = vec![];
        for m in others {
            if self.overlaps(m) {
                overlaps.push(m);
            }
        }

        if overlaps.is_empty() {
            return vec![self.clone()];
        }

        overlaps.sort_by_key(|m| m.source);

        let mut out_maps = vec![];

        let overlap_num = overlaps[0].source;
        if overlap_num > self.dest {
            // if we are not overlapping the start of the original map
            // then the first mapping is the first section of the original map
            out_maps.push(StoDMap {
                source: self.source,
                dest: self.dest,
                length: overlap_num - self.dest,
            })
        }

        let mut index = 0;
        loop {
            let other = &overlaps[index];
            out_maps.push(self.squash(other));

            // have we reached the end of the original map?
            if other.source + other.length >= self.dest + self.length {
                break;
            }

            // is this the last overlapping map?
            if index + 1 == overlaps.len() {
                // need to write the last non-overlapped portion of the original map back out
                let new_length = self.length - (other.source + other.length - self.dest);
                let new_source = self.source + (self.length - new_length);
                let new_dest = self.dest + (self.length - new_length);
                out_maps.push(StoDMap {
                    source: new_source,
                    dest: new_dest,
                    length: new_length,
                });
                break;
            }

            // is there a gap before the next map?
            if other.source + other.length < overlaps[index + 1].source {
                // write out the unoverlapped section
                let new_length = overlaps[index + 1].source - (other.source + other.length);
                let new_dest = other.source + other.length;
                let new_source = new_dest + (self.source - self.dest);
                out_maps.push(StoDMap {
                    source: new_source,
                    dest: new_dest,
                    length: new_length,
                })
            }

            index += 1;
        }

        out_maps
    }

    fn overlaps(&self, other: &StoDMap) -> bool {
        if other.source >= self.dest && other.source < self.dest + self.length {
            return true;
        }
        if other.source < self.dest && other.source + other.length > self.dest {
            return true;
        }
        false
    }

    fn from_string(input: &str) -> StoDMap {
        let in_vec: Vec<&str> = input.split(' ').filter(|s| !s.is_empty()).collect();
        let dest = in_vec[0]
            .parse::<i64>()
            .expect("from_string() expects a string containing 3 numbers separated by whitespace");
        let source = in_vec[1]
            .parse::<i64>()
            .expect("from_string() expects a string containing 3 numbers separated by whitespace");
        let length = in_vec[2]
            .parse::<i64>()
            .expect("from_string() expects a string containing 3 numbers separated by whitespace");

        StoDMap {
            source,
            dest,
            length,
        }
    }
}

fn solution(input_str: &str) -> (i64, i64) {
    let seeds: Vec<i64> = input_str
        .lines()
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<i64>())
        .collect();
    let mut seed_mappings_p1: Vec<StoDMap> = seeds
        .iter()
        .map(|x| StoDMap {
            source: *x,
            dest: *x,
            length: 1,
        })
        .collect();
    let mut seed_mappings_p2: Vec<StoDMap> = seeds
        .chunks(2)
        .map(|x| StoDMap {
            source: x[0],
            dest: x[0],
            length: x[1],
        })
        .collect();

    let mut curr_maps = vec![];

    for line in input_str.lines().skip(3) {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            let mut new_mappings = vec![];
            for m in &seed_mappings_p1 {
                new_mappings.append(&mut m.squash_many(&curr_maps));
            }
            seed_mappings_p1 = new_mappings;
            new_mappings = vec![];

            for m in &seed_mappings_p2 {
                new_mappings.append(&mut m.squash_many(&curr_maps));
            }
            seed_mappings_p2 = new_mappings;
            curr_maps = vec![];
            continue;
        }

        curr_maps.push(StoDMap::from_string(line.trim()));
    }

    let mut new_mappings = vec![];
    for m in &seed_mappings_p2 {
        new_mappings.append(&mut m.squash_many(&curr_maps));
    }
    seed_mappings_p2 = new_mappings;

    let p1out = seed_mappings_p1
        .iter()
        .fold(i64::MAX, |m, x| min(m, x.dest));
    let p2out = seed_mappings_p2
        .iter()
        .fold(i64::MAX, |m, x| min(m, x.dest));

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

fn old_solution(input_str: &str) -> (i64, i64) {
    let seeds: Vec<i64> = input_str
        .lines()
        .next()
        .expect("")
        .split_once(':')
        .expect("")
        .1
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<i64>())
        .collect();
    let seeds_p2: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|x| Range {
            start: x[0],
            end: x[1] + x[0],
        })
        .collect();

    let mut reverse_maps: Vec<HashMap<Range<i64>, i64>> = vec![];
    let mut curr_rev_map: HashMap<Range<i64>, i64> = HashMap::new();

    for line in input_str.lines().skip(3) {
        if line.contains("map") {
            reverse_maps.push(curr_rev_map);
            curr_rev_map = HashMap::new();
            continue;
        }

        let range_vec: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
        if range_vec.is_empty() {
            continue;
        }

        assert_eq!(range_vec.len(), 3);

        let dest_start: i64 = range_vec[0].parse().expect("");
        let source_start: i64 = range_vec[1].parse().expect("");
        let length: i64 = range_vec[2].parse().expect("");

        curr_rev_map.insert(
            Range {
                start: dest_start,
                end: dest_start + length,
            },
            source_start - dest_start,
        );
    }

    reverse_maps.push(curr_rev_map);

    reverse_maps.reverse();
    let mut min_loc = 0;
    let mut min_loc1 = -1;
    let mut min_loc2 = -1;
    loop {
        let mut val = min_loc;
        let mut trans = vec![];
        for m in &reverse_maps {
            for r in m.keys() {
                if r.contains(&val) {
                    val += m[r];
                    break;
                }
            }
            trans.push(val);
        }
        if seeds.contains(&val) && min_loc1 < 0 {
            min_loc1 = min_loc;
            println!("min_loc1 = {}, transitions: {:?}", min_loc1, trans);
        }
        for r in &seeds_p2 {
            if r.contains(&val) && min_loc2 < 0 {
                min_loc2 = min_loc;
                println!("min_loc2 = {}, transitions: {:?}", min_loc2, trans);
            }
        }
        if min_loc1 >= 0 && min_loc2 >= 0 {
            break;
        }
        min_loc += 1;
    }

    (min_loc1, min_loc2)
}

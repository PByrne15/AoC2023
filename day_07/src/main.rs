use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fs,
    iter::zip,
};

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<char>,
    rank: i32,
}

impl Hand {
    // Calculate a ranking for a hand of cards
    // Rankings are (higher is stronger):
    // 6: Five of a kind
    // 5: Four of a kind
    // 4: Full house
    // 3: Three of a kind
    // 2: Two pair
    // 1: One pair
    // 0: High Card
    fn calc_rank(cards: &Vec<char>) -> i32 {
        assert!(cards.len() == 5);

        let card_map = {
            let mut tmp_map = HashMap::new();
            for c in cards {
                if tmp_map.contains_key(c) {
                    tmp_map.insert(c, tmp_map[c] + 1);
                } else {
                    tmp_map.insert(c, 1);
                }
            }
            tmp_map
        };

        // Five of a kind
        if card_map.values().filter(|&&v| v == 5).count() == 1 {
            return 6;
        }
        // Four of a kind
        if card_map.values().filter(|&&v| v == 4).count() == 1 {
            return 5;
        }
        if card_map.values().filter(|&&v| v == 3).count() == 1 {
            // Full house
            if card_map.values().filter(|&&v| v == 2).count() == 1 {
                return 4;
            }
            // Three of a kind
            return 3;
        }
        // Two pair
        if card_map.values().filter(|&&v| v == 2).count() == 2 {
            return 2;
        }

        // One pair
        if card_map.values().filter(|&&v| v == 2).count() == 1 {
            return 1;
        }

        // High card
        0
    }

    fn from_string(str_in: &str) -> Hand {
        assert!(str_in.len() == 5);
        let cards = str_in.chars().collect();
        let rank = Hand::calc_rank(&cards);

        Hand { cards, rank }
    }

    fn cardwise_cmp(&self, other: &Hand) -> Ordering {
        for (c1, c2) in zip(&self.cards, &other.cards) {
            if c1 == c2 {
                continue;
            }
            if c1.is_numeric() && c2.is_numeric() {
                return c1.cmp(c2);
            }
            if !c1.is_numeric() && c2.is_numeric() {
                return Ordering::Greater;
            }
            if c1.is_numeric() && !c2.is_numeric() {
                return Ordering::Less;
            }
            match c1 {
                'A' => return Ordering::Greater,
                'T' => return Ordering::Less,
                'K' => {
                    if *c2 == 'A' {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
                'Q' => {
                    if *c2 == 'A' || *c2 == 'K' {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
                'J' => {
                    if *c2 == 'T' {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                }
                x => {
                    panic!("Found unexpected card {}", x);
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cardwise_cmp(other),
        }
    }
}

#[derive(Debug, Eq)]
struct HandP2 {
    cards: Vec<char>,
    rank: i32,
}

impl HandP2 {
    // Calculate a ranking for a hand of cards
    // Rankings are (higher is stronger):
    // 6: Five of a kind
    // 5: Four of a kind
    // 4: Full house
    // 3: Three of a kind
    // 2: Two pair
    // 1: One pair
    // 0: High Card
    fn calc_rank(cards: &Vec<char>) -> i32 {
        assert!(cards.len() == 5);

        let mut card_map = cards.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let j = if card_map.contains_key(&'J') {
            let v = card_map[&'J'];
            card_map.remove(&'J');
            v
        } else {
            0
        };

        // Five of a kind
        if card_map.is_empty() || card_map.values().filter(|&&v| v + j == 5).count() > 0 {
            return 6;
        }
        // Four of a kind
        if card_map.values().filter(|&&v| v + j == 4).count() > 0 {
            return 5;
        }
        if card_map.values().filter(|&&v| v + j == 3).count() > 0 {
            // Full house
            if (j == 0 && card_map.values().filter(|&&v| v == 2).count() == 1)
                || card_map.values().filter(|&&v| v == 2).count() == 2
            {
                return 4;
            }
            // Three of a kind
            return 3;
        }
        // Two pair
        if card_map.values().filter(|&&v| v == 2).count() == 2 {
            return 2;
        }

        // One pair
        if card_map.values().filter(|&&v| v + j == 2).count() > 0 {
            return 1;
        }

        // High card
        0
    }

    fn from_string(str_in: &str) -> HandP2 {
        assert!(str_in.len() == 5);
        let cards = str_in.chars().collect();

        let rank = HandP2::calc_rank(&cards);

        HandP2 { cards, rank }
    }

    fn cardwise_cmp(&self, other: &HandP2) -> Ordering {
        for (c1, c2) in zip(&self.cards, &other.cards) {
            if c1 == c2 {
                continue;
            }
            if *c1 == 'J' {
                return Ordering::Less;
            }
            if *c2 == 'J' {
                return Ordering::Greater;
            }
            if c1.is_numeric() && c2.is_numeric() {
                return c1.cmp(c2);
            }
            if !c1.is_numeric() && c2.is_numeric() {
                return Ordering::Greater;
            }
            if c1.is_numeric() && !c2.is_numeric() {
                return Ordering::Less;
            }
            match c1 {
                'A' => return Ordering::Greater,
                'T' => return Ordering::Less,
                'K' => {
                    if *c2 == 'A' {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
                'Q' => {
                    if *c2 == 'A' || *c2 == 'K' {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
                x => {
                    panic!("Found unexpected card {}", x);
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for HandP2 {
    fn eq(&self, other: &HandP2) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for HandP2 {
    fn partial_cmp(&self, other: &HandP2) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandP2 {
    fn cmp(&self, other: &HandP2) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cardwise_cmp(other),
        }
    }
}

fn solution(input_str: &str) -> (i32, i32) {
    let mut ordered_map = BTreeMap::new();
    let mut ordered_map_p2 = BTreeMap::new();

    for line in input_str.lines() {
        let (key_str, value) = line.split_once(' ').expect("");

        let key = Hand::from_string(key_str);
        let keyp2 = HandP2::from_string(key_str);

        let value = value.parse::<i32>().expect("");
        ordered_map.insert(key, value);
        ordered_map_p2.insert(keyp2, value);
    }

    let mut score = 0;
    for (rank, hand) in ordered_map.iter().enumerate() {
        score += (rank as i32 + 1) * hand.1;
    }

    let mut score_p2 = 0;
    for (rank, hand) in ordered_map_p2.iter().enumerate() {
        score_p2 += (rank as i32 + 1) * hand.1;
    }

    (score, score_p2)
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

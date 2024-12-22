use adventofcode2024 as aoc;
use std::collections::{HashMap, HashSet};

fn main() {
    let numbers = aoc::input::parse_lines::<u64>("day22");

    let mut sum1 = 0;
    let mut combinations_sums: HashMap<[i8; 4], u64> = HashMap::new();

    for mut num in numbers {
        let mut diffs = [0i8; 4];
        let mut bananas_prev = 0;
        let mut combinations_found: HashSet<[i8; 4]> = HashSet::new();

        for i in 0..2000 {
            num = (num ^ (num * 64)) % 16777216;
            num = (num ^ (num / 32)) % 16777216;
            num = (num ^ (num * 2048)) % 16777216;

            let bananas = (num % 10) as i8;
            if i >= 1 {
                diffs.rotate_left(1);
                diffs[3] = bananas - bananas_prev;
            }
            bananas_prev = bananas;

            if i >= 4 && combinations_found.insert(diffs.clone()) {
                *combinations_sums.entry(diffs.clone()).or_default() += bananas as u64;
            }
        }

        sum1 += num;
    }

    let sum2 = combinations_sums
        .into_iter()
        .map(|(_, count)| count)
        .max()
        .unwrap();

    println!("Part 1: sum={sum1}");
    println!("Part 2: sum={sum2}");
}

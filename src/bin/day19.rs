use adventofcode2024 as aoc;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct StrByLen(String);

fn main() {
    let (substrs, target_strs) = read_input();
    let mut sum1 = 0;
    let mut sum2 = 0;

    for target_str in &target_strs {
        let sum = num_combinations(target_str, &substrs);
        if sum > 0 {
            sum1 += 1;
            sum2 += sum;
        }
    }

    println!("Part 1: valid combinations={sum1}");
    println!("Part 2: valid combinations={sum2}");
}

fn num_combinations(target_str: &str, substrs: &[String]) -> u64 {
    let mut total = 0;
    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(StrByLen("".to_string()));

    // It is important to process the substrings in ascending order by string length.
    // This way, when we continue creating combinations from a certain substring, we
    // already have the correct count of how many valid combinations exist to create
    // that substring.
    while let Some(StrByLen(mut str)) = queue.pop() {
        let count = *seen.get(&str).unwrap_or(&1);
        let str_initial_len = str.len();

        for substr in substrs {
            // Do this to not allocate a new string each iteration. Do it only if added to the queue.
            str.truncate(str_initial_len);
            str.push_str(substr);

            if str == target_str {
                // found
                total += count;
                continue;
            } else if str.len() >= target_str.len() || !target_str.starts_with(&str) {
                continue;
            } else if let Some(prev_count) = seen.get_mut(&str) {
                // We had already built this substring before. We already knew `prev_count`
                // different combinations to build it. Now we add `count` new ways to build it.
                *prev_count += count;
                continue;
            } else {
                seen.insert(str.clone(), count);
                queue.push(StrByLen(str.clone()));
            }
        }
    }

    total
}

fn read_input() -> (Vec<String>, Vec<String>) {
    let mut lines = aoc::input::read_lines("day19");
    let substrs: Vec<String> = lines
        .by_ref()
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect();
    let target_strs: Vec<String> = lines.skip(1).collect();
    (substrs, target_strs)
}

impl Ord for StrByLen {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.len().cmp(&self.0.len()) // inverted so it's a min heap: lower str length first
    }
}

impl PartialOrd for StrByLen {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

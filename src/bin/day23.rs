use adventofcode2024 as aoc;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut connected_pairs: HashMap<String, Vec<String>> = HashMap::new();

    for mut connected_pair in aoc::input::read_tokens_split_str("day23", "-") {
        connected_pair.sort();
        let to = connected_pair.pop().unwrap();
        let from = connected_pair.pop().unwrap();

        assert!(connected_pair.is_empty());
        assert!(!connected_pairs
            .get(&from)
            .is_some_and(|dsts| dsts.contains(&to)));

        connected_pairs.entry(from).or_default().push(to);
    }

    let mut sum = 0;
    let mut triple_conns_found = HashSet::new();
    for (from, to_list) in &connected_pairs {
        for i in 0..to_list.len() {
            let c1 = &to_list[i];
            for c2 in &to_list[i + 1..] {
                if !any_starts_with_t(from, c1, c2) {
                    continue;
                }
                let mut pair = [c1, c2];
                pair.sort();
                if connected_pairs
                    .get(pair[0])
                    .is_some_and(|to_list| to_list.contains(pair[1]))
                {
                    let mut three = [from, c1, c2];
                    three.sort();
                    let conn_str: String = three.into_iter().flat_map(|c| c.chars()).collect();
                    if triple_conns_found.insert(conn_str) {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: sum={sum}");

    // This solution is not correct: only one group is created for each computer,
    // but it might be part of various groups, actually. It happens that it found
    // the right answer, though.
    let mut longest_group = "".to_string();
    for (from, to_list) in &connected_pairs {
        let mut group = Vec::new();

        for computer1 in to_list {
            if group
                .iter()
                .all(|computer2| are_connected(&connected_pairs, computer1, computer2))
            {
                group.push(computer1.clone());
            }
        }

        group.push(from.clone());
        group.sort();

        let group_str = group.join(",");
        if group_str.len() > longest_group.len() {
            longest_group = group_str;
        }
    }
    println!("Part 2: longest group={longest_group}");
}

fn are_connected(connected_pairs: &HashMap<String, Vec<String>>, c1: &String, c2: &String) -> bool {
    let from = c1.min(c2);
    let to = c1.max(c2);
    connected_pairs
        .get(from)
        .is_some_and(|to_list| to_list.contains(to))
}

fn any_starts_with_t(a: &str, b: &str, c: &str) -> bool {
    a.chars().next().is_some_and(|ch| ch == 't')
        || b.chars().next().is_some_and(|ch| ch == 't')
        || c.chars().next().is_some_and(|ch| ch == 't')
}

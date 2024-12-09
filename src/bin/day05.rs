use adventofcode2024 as aoc;
use std::collections::HashMap;

type Rules = HashMap<i32, Vec<i32>>;
type Pages = Vec<Vec<i32>>;

fn main() {
    let (before_rules, pages_lines) = parse_input();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for mut pages_line in pages_lines {
        let mut ordered = true;

        for i in 0..pages_line.len() - 1 {
            let page = pages_line[i];

            if let Some(page_rules) = before_rules.get(&page) {
                let next_pages = &pages_line[i + 1..];
                if next_pages
                    .iter()
                    .any(|next_page| page_rules.contains(next_page))
                {
                    ordered = false;
                    break;
                }
            }
        }

        if ordered {
            sum1 += pages_line[pages_line.len() / 2];
        } else {
            pages_line.sort_by(|a, b| compare_pages(&before_rules, *a, *b));
            sum2 += pages_line[pages_line.len() / 2];
        }
    }

    println!("Part 1: sum={sum1}");
    println!("Part 2: sum={sum2}");
}

fn compare_pages(before_rules: &Rules, a: i32, b: i32) -> std::cmp::Ordering {
    if let Some(page_rules) = before_rules.get(&a) {
        if page_rules.contains(&b) {
            return std::cmp::Ordering::Greater;
        }
    }
    if let Some(page_rules) = before_rules.get(&b) {
        if page_rules.contains(&a) {
            return std::cmp::Ordering::Less;
        }
    }
    std::cmp::Ordering::Equal
}

fn parse_input() -> (Rules, Pages) {
    let mut parsing_rules = true;
    let mut before_rules = HashMap::new();
    let mut pages_lines = Vec::new();

    for line in aoc::input::read_lines("day05") {
        if line == "" {
            parsing_rules = false;
        } else if parsing_rules {
            insert_rule(&mut before_rules, line);
        } else {
            pages_lines.push(parse_pages(line));
        }
    }

    (before_rules, pages_lines)
}

fn insert_rule(rules: &mut Rules, line: String) {
    let mut split = line.split('|');
    let first = split.next().unwrap().parse::<i32>().unwrap();
    let second = split.next().unwrap().parse::<i32>().unwrap();
    rules
        .entry(second)
        .or_insert_with(|| Vec::new())
        .push(first);
}

fn parse_pages(line: String) -> Vec<i32> {
    line.split(",").map(|v| v.parse::<i32>().unwrap()).collect()
}

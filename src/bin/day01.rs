use adventofcode2024 as aoc;
use std::iter::zip;

fn main() {
    let mut sum = 0;
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in aoc::input::read_tokens_split_str::<i32>("day01", "   ") {
        assert!(line.len() == 2);
        list1.push(line[0]);
        list2.push(line[1]);
    }

    list1.sort();
    list2.sort();

    for (n1, n2) in zip(&list1, &list2) {
        sum += (*n1 - *n2).abs();
    }

    println!("Part 1: sum={sum}");

    sum = 0;
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < list1.len() {
        let num = list1[idx1];

        while idx2 < list2.len() && list2[idx2] < num {
            idx2 += 1;
        }
        if idx2 >= list2.len() {
            break;
        }
        while idx2 < list2.len() && list2[idx2] == num {
            sum += num;
            idx2 += 1;
        }
        idx1 += 1;
    }

    println!("Part 2: sum={sum}");
}

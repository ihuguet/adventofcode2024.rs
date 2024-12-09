use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day03.txt").unwrap();

    let mut sum = 0;
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for mul_match in mul_regex.captures_iter(&input) {
        let (_, [n1, n2]) = mul_match.extract();
        let n1 = n1.parse::<i32>().unwrap();
        let n2 = n2.parse::<i32>().unwrap();
        sum += n1 * n2;
    }
    println!("Part 1: sum={sum}");

    let mut sum = 0;
    let mut mul_enable = true;
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for mul_match in mul_regex.captures_iter(&input) {
        match mul_match.get(0).unwrap().as_str() {
            "do()" => mul_enable = true,
            "don't()" => mul_enable = false,
            _ => {
                if mul_enable {
                    let n1 = mul_match.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let n2 = mul_match.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    sum += n1 * n2;
                }
            }
        }
    }
    println!("Part 2: sum={sum}");
}

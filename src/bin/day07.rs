use adventofcode2024 as aoc;
use std::str::FromStr;

struct Op {
    result: u64,
    operands: Vec<u64>,
}

fn main() {
    let ops = aoc::input::parse_lines::<Op>("day07");
    let mut sum1 = 0;
    let mut sum2 = 0;

    for op in ops {
        let result = op.result;
        let accum = op.operands[0];
        let rest = &op.operands[1..];

        if operation_equals_part1(accum, rest, result) {
            sum1 += result;
        }
        if operation_equals_part2(accum, rest, result) {
            sum2 += result;
        }
    }

    println!("Part 1: valid values sum={sum1}");
    println!("Part 2: valid values sum={sum2}");
}

fn operation_equals_part1(accum: u64, rest: &[u64], result: u64) -> bool {
    if rest.len() == 0 {
        accum == result
    } else {
        let accum_sum = accum + rest[0];
        let accum_mul = accum * rest[0];
        let rest = &rest[1..];
        operation_equals_part1(accum_sum, rest, result)
            || operation_equals_part1(accum_mul, rest, result)
    }
}

fn operation_equals_part2(accum: u64, rest: &[u64], result: u64) -> bool {
    if rest.len() == 0 {
        accum == result
    } else {
        let accum_sum = accum + rest[0];
        let accum_mul = accum * rest[0];
        let accum_cat = (accum.to_string() + &rest[0].to_string())
            .parse::<u64>()
            .unwrap();
        let rest = &rest[1..];
        operation_equals_part2(accum_sum, rest, result)
            || operation_equals_part2(accum_mul, rest, result)
            || operation_equals_part2(accum_cat, rest, result)
    }
}

impl FromStr for Op {
    type Err = aoc::input::ParseAoCInputError<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let result = split
            .next()
            .ok_or_else(|| aoc::input::ParseAoCInputError::new("Too few elements"))?
            .parse::<u64>()?;
        let operands = split
            .next()
            .ok_or_else(|| aoc::input::ParseAoCInputError::new("Too few elements"))?
            .trim()
            .split(" ")
            .map(|v| v.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;
        Ok(Op { result, operands })
    }
}

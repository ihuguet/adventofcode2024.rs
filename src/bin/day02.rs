use adventofcode2024 as aoc;

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in aoc::input::read_tokens_split_chars::<i32>("day02", &[' ']) {
        if valid(&line) {
            sum1 += 1;
            sum2 += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut line_elem_removed = line.clone();
            line_elem_removed.remove(i);
            if valid(&line_elem_removed) {
                sum2 += 1;
                break;
            }
        }
    }

    println!("Part 1: sum={sum1}");
    println!("Part 2: sum={sum2}");
}

fn valid(numbers: &[i32]) -> bool {
    let sign = (numbers[0] - numbers[1]).signum();
    numbers.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        return diff.signum() == sign && diff.abs() >= 1 && diff.abs() <= 3;
    })
}

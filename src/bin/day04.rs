use adventofcode2024 as aoc;

fn main() {
    let lines: Vec<Vec<char>> = aoc::input::read_lines("day04")
        .map(|line| line.chars().collect())
        .collect();
    let y_len = lines.len() as i32;
    let x_len = lines[0].len() as i32;

    let mut sum = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            for y_diff in [-1, 0, 1] {
                for x_diff in [-1, 0, 1] {
                    if (y_diff == 0 && x_diff == 0)
                        || (y + 3 * y_diff < 0 || y + 3 * y_diff >= y_len)
                        || (x + 3 * x_diff < 0 || x + 3 * x_diff >= x_len)
                    {
                        continue;
                    }

                    let mut chars = [' '; 4];
                    for i in 0..4 {
                        let y = (y + i as i32 * y_diff) as usize;
                        let x = (x + i as i32 * x_diff) as usize;
                        chars[i] = lines[y][x];
                    }
                    if chars == ['X', 'M', 'A', 'S'] {
                        sum += 1;
                    }
                }
            }
        }
    }
    println!("Part 1: sum={sum}");

    let mut sum = 0;
    for y in 1..y_len as usize - 1 {
        for x in 1..x_len as usize - 1 {
            let chars1 = (lines[y - 1][x - 1], lines[y][x], lines[y + 1][x + 1]);
            let chars2 = (lines[y + 1][x - 1], lines[y][x], lines[y - 1][x + 1]);
            if (chars1 == ('M', 'A', 'S') || chars1 == ('S', 'A', 'M'))
                && (chars2 == ('M', 'A', 'S') || chars2 == ('S', 'A', 'M'))
            {
                sum += 1;
            }
        }
    }
    println!("Part 2: sum={sum}");
}

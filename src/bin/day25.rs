use adventofcode2024 as aoc;

fn main() {
    let (locks, keys) = parse_input();
    let mut sum = 0;

    for lock in locks {
        for key in &keys {
            if (0..5).all(|i| lock[i] + key[i] <= 7) {
                sum += 1;
            }
        }
    }

    println!("Pairs of keys and locks that fit = {sum}");
}

fn parse_input() -> (Vec<[u32; 5]>, Vec<[u32; 5]>) {
    let mut lines = aoc::input::read_lines("day25");
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    loop {
        let block: Vec<Vec<char>> = lines
            .by_ref()
            .map_while(|l| (l != "").then_some(l.chars().collect()))
            .collect();

        if block.is_empty() {
            break;
        }

        let mut heights = [0; 5];
        for col in 0..5 {
            for row in 0..7 {
                if block[row][col] == '#' {
                    heights[col] += 1;
                }
            }
        }

        match block[0][0] {
            '#' => locks.push(heights),
            '.' => keys.push(heights),
            _ => panic!(),
        }
    }

    (locks, keys)
}

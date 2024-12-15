use adventofcode2024 as aoc;
use std::{collections::HashSet, str::FromStr};

const GRID_WIDTH: i64 = 101;
const GRID_HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

fn main() {
    let robots: Vec<Robot> = aoc::input::parse_lines("day14").collect();

    // We cannot find the solution to part 2 automatically. Call to this binary
    // with --print-all to print all the posible positions of the robots. Pipe
    // the output to a file and search "#############" to find the xmas tree.
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("--print-all") {
        part2_print_all(&robots);
        return;
    }

    println!("Part 1: safety factor={}", part1(&robots));
    println!("Part 2:");
    part2(&robots);
}

fn part1(robots: &[Robot]) -> i64 {
    let mut quadrants_count = [[0, 0], [0, 0]];

    for robot in robots {
        let (x, y) = calc_future_pos(&robot, 100);

        let quadrant_x = if x < GRID_WIDTH / 2 {
            0
        } else if x > GRID_WIDTH / 2 {
            1
        } else {
            continue;
        };

        let quadrant_y = if y < GRID_HEIGHT / 2 {
            0
        } else if y > GRID_HEIGHT / 2 {
            1
        } else {
            continue;
        };

        quadrants_count[quadrant_x][quadrant_y] += 1;
    }

    quadrants_count.as_flattened().iter().product()
}

fn part2(robots: &[Robot]) {
    // Seconds obtained running this program with --print-all, piping to a file
    // and searching in that file for a long "###########" string.
    print_grid(robots, 7861);
}

fn part2_print_all(robots: &[Robot]) {
    let mut prev_positions: Vec<HashSet<(i64, i64)>> = Vec::new();

    for seconds in 0.. {
        let positions = print_grid(robots, seconds);
        if prev_positions.iter().any(|pp| *pp == positions) {
            println!("{seconds}");
            break;
        }
        prev_positions.push(positions);
    }
}

fn calc_future_pos(robot: &Robot, seconds: i64) -> (i64, i64) {
    let mut x = (robot.px + robot.vx * seconds) % GRID_WIDTH;
    if x < 0 {
        x += GRID_WIDTH;
    }

    let mut y = (robot.py + robot.vy * seconds) % GRID_HEIGHT;
    if y < 0 {
        y += GRID_HEIGHT;
    }

    (x, y)
}

fn print_grid(robots: &[Robot], seconds: i64) -> HashSet<(i64, i64)> {
    let mut positions = HashSet::new();

    for robot in robots {
        let pos = calc_future_pos(&robot, seconds);
        positions.insert(pos);
    }

    println!("----------- seconds: {seconds} -----------");
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();

    positions
}

impl FromStr for Robot {
    type Err = aoc::input::ParseAoCInputError<Robot>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(&['=', ',', ' ']).collect();
        Ok(Robot {
            px: tokens[1].parse()?,
            py: tokens[2].parse()?,
            vx: tokens[4].parse()?,
            vy: tokens[5].parse()?,
        })
    }
}

use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};

fn main() {
    let grid = parse_input();
    let mut grid_results1 = vec![vec![false; grid[0].len()]; grid.len()];
    let mut grid_results2 = vec![vec![false; grid[0].len()]; grid.len()];

    for (point, &ch) in Grid::<isize>::iter_grid(&grid) {
        if ch != '.' {
            for (point2, &ch2) in Grid::<isize>::iter_grid(&grid) {
                if point != point2 && ch == ch2 {
                    let diff = point - point2;

                    // part 1
                    let point_result = point + diff;
                    if grid_results1.contains_point(point_result) {
                        grid_results1[point_result] = true;
                    }

                    // part 2
                    for i in 0.. {
                        let point_result = point + diff * i;
                        if grid_results2.contains_point(point_result) {
                            grid_results2[point_result] = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    let sum = grid_results1
        .iter_grid()
        .filter(|(_, v): &(Point, &bool)| **v)
        .count();
    println!("Part 1: antinodes num={sum}");

    let sum = grid_results2
        .iter_grid()
        .filter(|(_, v): &(Point, &bool)| **v)
        .count();
    println!("Part 2: antinodes num={sum}");
}

fn parse_input() -> VecGrid<char> {
    aoc::input::read_lines("day08")
        .map(|line| line.chars().collect())
        .collect()
}

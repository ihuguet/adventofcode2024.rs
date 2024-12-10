use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let grid = parse_input();

    let mut queue: VecDeque<(Point, Point)> = VecDeque::new();
    let mut paths_count: HashMap<Point, u32> = HashMap::new();
    let mut reachable_peaks: HashMap<Point, HashSet<Point>> = HashMap::new();

    for (point, val) in grid.iter_grid() {
        if *val == 0 {
            queue.push_back((point, point));
            paths_count.insert(point, 0);
            reachable_peaks.insert(point, HashSet::new());
        }
    }

    while let Some((orig, point)) = queue.pop_front() {
        let val = grid[point];
        for point_next in grid.adjacents_4(point) {
            let val_next = grid[point_next];
            if val == 8 && val_next == 9 {
                *paths_count.get_mut(&orig).unwrap() += 1;
                reachable_peaks.get_mut(&orig).unwrap().insert(point_next);
            } else if val_next == val + 1 {
                queue.push_back((orig, point_next));
            }
        }
    }

    let sum1: u32 = reachable_peaks.iter().map(|(_, v)| v.len() as u32).sum();
    let sum2: u32 = paths_count.iter().map(|(_, v)| v).sum();
    println!("Part 1: paths={sum1}");
    println!("Part 2: score={sum2}");
}

fn parse_input() -> VecGrid<u32> {
    aoc::input::read_lines("day10")
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

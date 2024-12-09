use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Obstacle,
    Unvisited,
    Visited,
}

fn main() {
    let (guard, grid) = parse_input();

    let (sum, visited_points) = part1(grid.clone(), guard);
    println!("Part 1: visited points={sum}");

    let sum = part2(grid, &visited_points, guard);
    println!("Part 2: loop forcing points={sum}");
}

fn part1(mut grid: VecGrid<Cell>, mut guard: Point<isize>) -> (usize, Vec<Point<isize>>) {
    let mut dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = Point::<isize>::from(dirs[0]);

    while grid.contains_point(guard) {
        if let Some(Cell::Obstacle) = grid.get_point(guard + dir) {
            dirs.rotate_left(1);
            dir = dirs[0].into();
        } else {
            grid[guard] = Cell::Visited;
            guard += dir;
        }
    }

    let visited_points: Vec<Point<isize>> = grid
        .iter_grid()
        .filter(|(_, cell): &(Point<isize>, _)| **cell == Cell::Visited)
        .map(|(coord, _): (Point<isize>, _)| coord)
        .collect();

    (visited_points.len(), visited_points)
}

fn part2(mut grid: VecGrid<Cell>, visited: &[Point<isize>], guard: Point<isize>) -> usize {
    let mut sum = 0;

    for &new_obstacle in visited {
        if new_obstacle != guard {
            grid[new_obstacle] = Cell::Obstacle;
            if is_loop(&grid, guard) {
                sum += 1;
            }
            grid[new_obstacle] = Cell::Unvisited;
        }
    }

    sum
}

fn is_loop(grid: &VecGrid<Cell>, mut guard: Point<isize>) -> bool {
    let mut dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = Point::<isize>::from(dirs[0]);
    let mut turned = HashSet::new();

    while grid.contains_point(guard) {
        if let Some(Cell::Obstacle) = grid.get_point(guard + dir) {
            // If we already visited this point facing the same direction: we're in a loop
            if turned.contains(&(guard, dir)) {
                return true;
            } else {
                turned.insert((guard, dir));
                dirs.rotate_left(1);
                dir = dirs[0].into();
            }
        } else {
            guard += dir;
        }
    }

    false
}

fn parse_input() -> (Point<isize>, VecGrid<Cell>) {
    let mut guard = (0, 0).into();
    let grid = aoc::input::read_lines("day06")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '#' => Cell::Obstacle,
                    '.' => Cell::Unvisited,
                    '^' => {
                        guard = (y as isize, x as isize).into();
                        Cell::Unvisited
                    }
                    _ => panic!("Unexpected char {ch}"),
                })
                .collect()
        })
        .collect();
    (guard, grid)
}

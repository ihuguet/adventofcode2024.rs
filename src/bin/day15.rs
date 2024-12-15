use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Box,
    Free,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell2 {
    Wall,
    BoxL,
    BoxR,
    Free,
}

fn main() {
    let (grid, moves, robot) = parse_input();
    let (grid2, robot2) = gen_grid2(&grid, robot);

    println!("Part 1: sum={}", part1(grid, robot, &moves));
    println!("Part 2: sum={}", part2(grid2, robot2, &moves));
}

fn part1(mut grid: VecGrid<Cell>, mut robot: Point<isize>, moves: &[Point<isize>]) -> usize {
    for &mov in moves {
        let mut dst = robot + mov;

        while grid[dst] == Cell::Box {
            dst += mov;
        }

        if grid[dst] == Cell::Free {
            robot += mov;
            if grid[robot] == Cell::Box {
                grid[robot] = Cell::Free;
                grid[dst] = Cell::Box;
            }
        }
    }

    grid.iter_grid()
        .filter(|(_, v): &(Point, _)| **v == Cell::Box)
        .map(|(p, _)| 100 * p.y + p.x)
        .sum()
}

fn part2(mut grid: VecGrid<Cell2>, mut robot: Point<isize>, moves: &[Point<isize>]) -> usize {
    let mut boxes_to_move = Vec::new();
    let mut cells_to_check = Vec::new();
    let mut line = Vec::new();

    for &mov in moves {
        if is_vertical(mov) {
            let mut can_move = true;
            boxes_to_move.clear();
            cells_to_check.clear();
            cells_to_check.push(robot + mov);

            // Do it line by line, so we can use them in reverse order later
            'outer: while !cells_to_check.is_empty() {
                line.clear();

                while let Some(p) = cells_to_check.pop() {
                    if grid[p] == Cell2::Wall {
                        can_move = false;
                        break 'outer;
                    } else if grid[p] == Cell2::BoxL {
                        line.push(p);
                        line.push(p + (0, 1).into());
                    } else if grid[p] == Cell2::BoxR {
                        line.push(p);
                        line.push(p + (0, -1).into());
                    }
                }

                // remove duplicates
                line.sort();
                line.dedup();

                // add the found boxes to the list of boxes to move
                boxes_to_move.extend_from_slice(&line);

                // add the points of the next lines to check
                cells_to_check.extend(line.iter().map(|&p| p + mov));
            }

            if can_move {
                for &p in boxes_to_move.iter().rev() {
                    grid[p + mov] = grid[p];
                    grid[p] = Cell2::Free;
                }
                robot += mov;
            }
        } else {
            // horizontal
            let mut dst = robot + mov;

            while matches!(grid[dst], Cell2::BoxL | Cell2::BoxR) {
                dst += mov;
            }

            if grid[dst] == Cell2::Free {
                while dst != robot {
                    grid[dst] = grid[dst - mov];
                    dst -= mov;
                }
                robot += mov;
            }
        }

        // print_all(&grid, robot);
    }

    grid.iter_grid()
        .filter(|(_, v): &(Point, _)| **v == Cell2::BoxL)
        .map(|(p, _)| 100 * p.y + p.x)
        .sum()
}

#[allow(dead_code)]
fn print_all(grid: &VecGrid<Cell2>, robot: Point<isize>) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let s = if y as isize == robot.y && x as isize == robot.x {
                "@"
            } else {
                match c {
                    Cell2::Wall => "#",
                    Cell2::BoxL => "[",
                    Cell2::BoxR => "]",
                    Cell2::Free => ".",
                }
            };
            print!("{}", s);
        }
        println!();
    }
}

fn is_vertical(mov: Point<isize>) -> bool {
    mov.x == 0 && mov.y != 0
}

fn parse_input() -> (VecGrid<Cell>, Vec<Point<isize>>, Point<isize>) {
    let mut lines = aoc::input::read_lines("day15");
    let mut robot = (0, 0).into();

    let grid = lines
        .by_ref()
        .enumerate()
        .map_while(|(y, line)| {
            if line != "" {
                Some(
                    line.chars()
                        .enumerate()
                        .map(|(x, ch)| {
                            if ch == '@' {
                                robot = (y as isize, x as isize).into();
                            }
                            match ch {
                                '#' => Cell::Wall,
                                'O' => Cell::Box,
                                '.' | '@' => Cell::Free,
                                _ => panic!(),
                            }
                        })
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();

    let mut moves = Vec::new();
    for line in lines {
        moves.extend(line.chars().map(|ch| match ch {
            '^' => Point::from((-1, 0)),
            '>' => Point::from((0, 1)),
            'v' => Point::from((1, 0)),
            '<' => Point::from((0, -1)),
            _ => panic!(),
        }));
    }

    (grid, moves, robot)
}

fn gen_grid2(grid: &VecGrid<Cell>, robot: Point<isize>) -> (VecGrid<Cell2>, Point<isize>) {
    let robot = (robot.y, robot.x * 2).into();
    let grid = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| match c {
                    Cell::Wall => [Cell2::Wall, Cell2::Wall],
                    Cell::Box => [Cell2::BoxL, Cell2::BoxR],
                    Cell::Free => [Cell2::Free, Cell2::Free],
                })
                .flatten()
                .collect()
        })
        .collect();

    (grid, robot)
}

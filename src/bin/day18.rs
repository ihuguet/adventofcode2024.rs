use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};
use std::collections::{BinaryHeap, HashMap, HashSet};

const GRID_W: usize = 71;
const GRID_H: usize = 71;

#[derive(Eq, PartialEq)]
struct State {
    steps: u64,
    pos: Point,
}

fn main() {
    let (obstacles, grid) = parse_input();
    println!("Part 1: min steps={}", part1(&grid));
    println!("Part 2: min steps={}", part2(&obstacles));
}

fn part1(grid: &VecGrid<bool>) -> u64 {
    let mut min_cost = u64::MAX;
    let mut visited: HashMap<Point, u64> = HashMap::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // The priority queue orders by distance to the destination, so it's a depth-first
    // search. This is convenient because the grid has many open areas. Better to find
    // a possible path quickly so we can start discarding less efficient paths as soon
    // as possible. A BFS algorithm would produce many possible routes in the open spaces,
    // most of them useless.
    queue.push(State::new((0, 0).into(), 0));
    visited.insert((0, 0).into(), 0);

    while let Some(State { steps, pos }) = queue.pop() {
        if pos == (70, 70).into() {
            if steps < min_cost {
                min_cost = steps;
            }
            continue;
        }

        let dist = Point { x: 70, y: 70 } - pos;
        if steps + dist.x as u64 + dist.y as u64 >= min_cost {
            continue;
        }

        if *visited.get(&pos).unwrap_or(&u64::MAX) < steps {
            continue;
        }

        for adj in grid.adjacents_4(pos).into_iter().filter(|adj| !grid[*adj]) {
            if *visited.get(&adj).unwrap_or(&u64::MAX) > steps + 1 {
                queue.push(State::new(adj, steps + 1));
                visited.insert(adj, steps + 1);
            }
        }
    }

    min_cost
}

fn part2(obstacles: &Vec<Point>) -> String {
    let mut grid = vec![vec![false; GRID_W]; GRID_H];
    let (dst_reachable, mut area) = get_partial_reachable_area(&grid);

    if !dst_reachable {
        panic!();
    }

    // Add obstacles one by one. If the obstacle falls inside the reachable area, we
    // need to recalculate that area, checking if the destination is still reachable
    // or not.
    // Note that we don't need the whole reachable area. We just need one path to the
    // destination. The smaller the area that we have, the less times we will need to
    // recalculate it. Because of that, we calculate the area searching depth-first, so
    // we obtain a smaller area.
    // A further improvement would be to limit the area to a single path, so we have to
    // recalculate even less times. But it already works fine as is.
    // Other improvement would be not to iterate the obstacles in a sequencial way, but
    // in a binsearch-like way. That would need to generate the grid each iteration,
    // though.
    for &obstacle in obstacles {
        grid[obstacle] = true;
        if area.contains(&obstacle) {
            let (dst_reachable, new_area) = get_partial_reachable_area(&grid);
            if !dst_reachable {
                return format!("{},{}", obstacle.x, obstacle.y);
            } else {
                area = new_area;
            }
        }
    }

    panic!();
}

fn get_partial_reachable_area(grid: &VecGrid<bool>) -> (bool, HashSet<Point>) {
    // It's called "partial" because it early returns when the destination point is found

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new((0, 0).into(), 0));
    let mut area = HashSet::new();
    area.insert((0, 0).into());

    while let Some(State { pos, .. }) = queue.pop() {
        for adj in grid.adjacents_4(pos).into_iter().filter(|adj| !grid[*adj]) {
            if area.insert(adj) {
                queue.push(State::new(adj, 0));
            }
            if adj == (GRID_H - 1, GRID_W - 1).into() {
                return (true, area);
            }
        }
    }

    (false, area)
}

fn parse_input() -> (Vec<Point>, VecGrid<bool>) {
    let obstacles: Vec<_> = aoc::input::read_lines("day18")
        .map(|line| {
            let mut s = line.split(",");
            let x: usize = s.next().unwrap().parse().unwrap();
            let y: usize = s.next().unwrap().parse().unwrap();
            Point { y, x }
        })
        .collect();

    let mut grid1 = vec![vec![false; GRID_W]; GRID_H];
    for &obstacle in &obstacles[..1024] {
        grid1[obstacle.y][obstacle.x] = true;
    }

    (obstacles, grid1)
}

impl State {
    fn new(pos: Point, steps: u64) -> State {
        State { pos, steps }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let dst = Point::from((GRID_H - 1, GRID_W - 1));
        let self_dist = dst - self.pos;
        let other_dist = dst - other.pos;
        // inverted so it's a min heap: less distance to dst first
        (other_dist.x as u64 + other_dist.y as u64).cmp(&(self_dist.x as u64 + self_dist.y as u64))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

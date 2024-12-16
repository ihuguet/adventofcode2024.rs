use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};
use std::collections::{BTreeSet, BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct State {
    pos: Point<isize>,
    dir: Point<isize>,
    cost: u32,
    path: Vec<Point<isize>>,
}

fn main() {
    let grid: VecGrid<char> = aoc::input::parse_chars_into("day16").collect();
    let (pos, _) = grid
        .iter_grid()
        .find(|&(_, &v): &(Point<isize>, &char)| v == 'S')
        .unwrap();

    let mut visited = HashMap::new();
    let mut best_paths = Vec::new();
    let mut min_cost = std::u32::MAX;
    let mut queue = BinaryHeap::new();
    queue.push(State::new(pos, (0, 1).into(), 0, vec![pos]));

    while let Some(state) = queue.pop() {
        let State {
            pos,
            dir,
            cost,
            path,
        } = state;

        // Is this the end point?
        if grid[pos] == 'E' {
            if cost < min_cost {
                min_cost = cost;
            }
            best_paths.push(path);
            continue;
        }

        // As queue is ordered by cost, if we already found a path with a lower
        // cost than this one, there are no more paths better than that.
        // IMPORTANT: part 2 requires to use > here, not >=.
        if cost > min_cost {
            break;
        }

        // If we already visited this point in the same direction but with a
        // lower cost, then that past visit was better than this one.
        // IMPORTANT: part 2 requires to use > here, not >=.
        if cost > *visited.get(&(pos, dir)).unwrap_or(&u32::MAX) {
            continue;
        } else {
            visited.insert((pos, dir), cost);
        }

        // Enqueue all possible next movements
        if grid[pos + dir] != '#' {
            let mut path_new = path.clone();
            path_new.push(pos + dir);
            queue.push(State::new(pos + dir, dir, cost + 1, path_new));
        }
        if grid[pos + turn_cw(dir)] != '#' {
            queue.push(State::new(pos, turn_cw(dir), cost + 1000, path.clone()));
        }
        if grid[pos + turn_ccw(dir)] != '#' {
            queue.push(State::new(pos, turn_ccw(dir), cost + 1000, path.clone()));
        }
    }

    let unique_points: BTreeSet<Point<isize>> = best_paths.into_iter().flatten().collect();
    println!("Part 1: min cost={min_cost}");
    println!("Part 2: points in a best path={}", unique_points.len());
}

fn turn_cw(dir: Point<isize>) -> Point<isize> {
    match dir.to_tuple() {
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        _ => panic!(),
    }
    .into()
}

fn turn_ccw(dir: Point<isize>) -> Point<isize> {
    match dir.to_tuple() {
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        (0, 1) => (-1, 0),
        _ => panic!(),
    }
    .into()
}

impl State {
    fn new(pos: Point<isize>, dir: Point<isize>, cost: u32, path: Vec<Point<isize>>) -> State {
        State {
            pos,
            dir,
            cost,
            path,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // inverted so it's a min heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

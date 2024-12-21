use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};

const MIN_SAVE: isize = 100;
const MAX_JUMP_DIST_1: isize = 2;
const MAX_JUMP_DIST_2: isize = 20;

fn main() {
    let grid: VecGrid<char> = aoc::input::parse_chars_into("day20").collect();

    let mut prev_pos = (-1, -1).into();
    let mut pos: Point<isize> = grid
        .iter_grid()
        .find_map(|(pos, ch)| (*ch == 'S').then_some(pos))
        .unwrap();
    let mut path = vec![pos];
    'outer: loop {
        for pos_next in grid.adjacents_4(pos) {
            if grid[pos_next] == '.' && pos_next != prev_pos {
                prev_pos = pos;
                pos = pos_next;
                path.push(pos);
                continue 'outer;
            } else if grid[pos_next] == 'E' {
                path.push(pos_next);
                break 'outer;
            }
        }
        panic!();
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (steps, &pos) in path.iter().enumerate() {
        for (steps_jump, &pos_jump) in path.iter().enumerate().skip(steps + MIN_SAVE as usize) {
            let dist = (pos_jump - pos).manhattan_dist();
            let steps_save = steps_jump as isize - steps as isize - dist;
            if steps_save >= MIN_SAVE {
                if dist <= MAX_JUMP_DIST_1 {
                    sum1 += 1;
                }
                if dist <= MAX_JUMP_DIST_2 {
                    sum2 += 1;
                }
            }
        }
    }

    println!("Part 1: cheats count={sum1}");
    println!("Part 2: cheats count={sum2}");
}

use adventofcode2024 as aoc;
use aoc::space_2D::{Grid, Point, VecGrid};
use std::collections::HashSet;

#[derive(Default)]
struct Region {
    area: usize,
    perim1: usize,
    perim2: usize,
}

fn main() {
    let grid: VecGrid<char> = aoc::input::parse_chars_into("day12").collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut regions = Vec::new();
    let mut tops = HashSet::new();
    let mut rights = HashSet::new();
    let mut bottoms = HashSet::new();
    let mut lefts = HashSet::new();
    let mut region_queue = Vec::new();

    for (point, _) in grid.iter_grid() {
        if visited[point] {
            continue;
        }

        let mut region = Region::default();
        region_queue.clear();
        region_queue.push(point);
        visited[point] = true;

        while let Some(point) = region_queue.pop() {
            let adjs: Vec<_> = grid
                .adjacents_4(point)
                .into_iter()
                .filter(|&p| grid[point] == grid[p])
                .collect();

            region.area += 1;
            region.perim1 += 4 - adjs.len();
            region.perim2 += check_side(&grid, &mut tops, point, (0, 1).into(), (-1, 0).into());
            region.perim2 += check_side(&grid, &mut rights, point, (1, 0).into(), (0, 1).into());
            region.perim2 += check_side(&grid, &mut bottoms, point, (0, -1).into(), (1, 0).into());
            region.perim2 += check_side(&grid, &mut lefts, point, (-1, 0).into(), (0, -1).into());

            for adj in adjs {
                if !visited[adj] {
                    visited[adj] = true;
                    region_queue.push(adj);
                }
            }
        }

        regions.push(region);
    }

    let price1: usize = regions
        .iter()
        .map(|region| region.area * region.perim1)
        .sum();
    let price2: usize = regions
        .into_iter()
        .map(|region| region.area * region.perim2)
        .sum();
    println!("Part 1: total price={}", price1);
    println!("Part 2: total price={}", price2);
}

fn check_side(
    grid: &VecGrid<char>,
    visited_sides: &mut HashSet<Point<isize>>,
    mut point: Point<isize>,
    side_dir: Point<isize>,
    oposite_dir: Point<isize>,
) -> usize {
    let mut ret = 0;
    let val = grid[point];

    while grid.get_point(point) == Some(&val) && grid.get_point(point + oposite_dir) != Some(&val) {
        if !visited_sides.insert(point) {
            return 0; // had been visited checking this same side yet
        }
        ret = 1;
        point += side_dir;
    }

    ret
}

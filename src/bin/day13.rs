use adventofcode2024 as aoc;
use aoc::space_2D::Point;

#[derive(Debug)]
struct Machine {
    move_a: Point<isize>,
    move_b: Point<isize>,
    target: Point<isize>,
}

fn main() {
    let mut machines = read_input();

    println!("Part 1: price={}", solve(&machines));

    for m in &mut machines {
        m.target.x += 10000000000000;
        m.target.y += 10000000000000;
    }
    println!("Part 2: price={}", solve(&machines));
}

fn solve(machines: &[Machine]) -> isize {
    let mut sum = 0;

    for machine in machines {
        // If we can reach the target pressing only B, it's the cheapest
        if machine.move_a == machine.move_b {
            if machine.target.y % machine.move_b.y == 0 && machine.target.x % machine.move_b.x == 0
            {
                sum += machine.target.y / machine.move_b.y;
            }
            continue; // We have already added the price. If not, the target was unreachable, anyway.
        }

        // Get the rect formula.
        // - If a move advances x=5, y=3, it will cross 5,3 if the rect starts from 0,0.
        // - We want to get the rect formula: cy*y = cx*x
        // - Start with y = c * x
        // - Replace x and y with the point that is crossed: 3 = c*5  ->  c = 3/5
        // - Formula: 5y = 3x
        // - Conclusion: formula is move_x*y = move_y*x
        let (rect_a_cy, rect_a_cx) = (machine.move_a.x, machine.move_a.y);
        let (rect_b_cy, rect_b_cx) = (machine.move_b.x, machine.move_b.y);

        // There is a single valid combination of button A + button B presses.
        // We can represent as rect_a from 0,0, and rect_b from `target`.
        // Calculate the offset so rect_b crosses `target`.
        // - cy*y = cx*x + c
        // - c = cy*y - cx*x
        // - Replace x,y with the coords of `target` anc calculate `c`.
        let rect_b_c = rect_b_cy * machine.target.y - rect_b_cx * machine.target.x;

        // Find where both rects cross (rect_a from 0,0 and rect_b from `target`):
        // - Do the `y` equality: cax*x/cay = (cbx*x + c)/cby
        // - cax*cby*x = cbx*cay*x + c*cay
        // - x = c*cay / (cax*cby - cbx*cay)
        let num = rect_b_c * rect_a_cy;
        let div = rect_a_cx * rect_b_cy - rect_b_cx * rect_a_cy;
        let cross_x = match num % div {
            0 => num / div,
            _ => continue, // No solution (only using decimals, not possible in this puzzle)
        };

        let num = rect_a_cx * cross_x;
        let div = rect_a_cy;
        let cross_y = match num % div {
            0 => num / div,
            _ => continue, // No solution (only using decimals, not possible in this puzzle)
        };

        // We have the point where the 2 rects cross. Just check how many times each button
        // has to be pressed.
        if cross_x % machine.move_a.x != 0
            || cross_y % machine.move_a.y != 0
            || (machine.target.x - cross_x) % machine.move_b.x != 0
            || (machine.target.y - cross_y) % machine.move_b.y != 0
        {
            continue; // No solution (only using decimals, not possible in this puzzle)
        }
        let button_a_count = cross_y / machine.move_a.y;
        let button_b_count = (machine.target.y - cross_y) / machine.move_b.y;

        // Sum the price
        sum += 3 * button_a_count + button_b_count;
    }

    sum
}

fn read_input() -> Vec<Machine> {
    let lines: Vec<_> = aoc::input::read_lines("day13").collect();
    let mut machines = Vec::new();

    for machine_lines in lines.chunks(4) {
        machines.push(Machine {
            move_a: parse_point(&machine_lines[0][10..]),
            move_b: parse_point(&machine_lines[1][10..]),
            target: parse_point(&machine_lines[2][7..]),
        });
    }

    machines
}

fn parse_point(s: &str) -> Point<isize> {
    let mut s = s.split(", ");
    let x = s.next().unwrap()[2..].parse().unwrap();
    let y = s.next().unwrap()[2..].parse().unwrap();
    (y, x).into()
}

const A0: i64 = 59590048;
const B0: i64 = 0;
const C0: i64 = 0;
const PROGRAM: [i64; 16] = [2, 4, 1, 5, 7, 5, 0, 3, 1, 6, 4, 3, 5, 5, 3, 0];

// const A0: i64 = 729;
// const B0: i64 = 0;
// const C0: i64 = 0;
// //const PROGRAM: [i64; 6] = [0, 1, 5, 4, 3, 0];
// const PROGRAM: [i64; 6] = [0, 3, 5, 4, 3, 0];

fn main() {
    println!("Part 1: output={}", part1());
    println!("Part 2: a={}", part2());
}

fn part1() -> String {
    let mut ip = 0;
    let mut a = A0;
    let mut b = B0;
    let mut c = C0;
    let mut out = Vec::new();

    while ip < PROGRAM.len() {
        let mut jump_to = ip + 2;
        match PROGRAM[ip] {
            0 => a = a / 2i64.pow(combo(PROGRAM[ip + 1], a, b, c) as u32),
            1 => b = b ^ PROGRAM[ip + 1],
            2 => b = combo(PROGRAM[ip + 1], a, b, c) % 8,
            3 if a == 0 => (),
            3 => jump_to = PROGRAM[ip + 1] as usize,
            4 => b = b ^ c,
            5 => out.push(combo(PROGRAM[ip + 1], a, b, c) % 8),
            6 => b = a / 2i64.pow(combo(PROGRAM[ip + 1], a, b, c) as u32),
            7 => c = a / 2i64.pow(combo(PROGRAM[ip + 1], a, b, c) as u32),
            v => panic!("Invalid instruction {v}"),
        }
        ip = jump_to;
    }

    let out_strs: Vec<String> = out.into_iter().map(|v| v.to_string()).collect();
    out_strs.join(",")
}

fn combo(op: i64, a: i64, b: i64, c: i64) -> i64 {
    match op {
        0..=3 => op,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid combo operator {op}"),
    }
}

fn part2() -> i64 {
    // PROGRAM (bits are shown as [b0..b1]):
    //   b = a[0..3]
    //   b = b ^ 5 = b ^ 0b101
    //   c = a / 2.pow(b)
    //   a = a / 8 = a >> 3
    //   b = b ^ 6 = b ^ 0b110
    //   b = b ^ c
    //   print b[0..3]
    //   if a != 0, repeat
    //
    // Shorter:
    //   c = a / 2.pow(a & 0x7 ^ 5)
    //   b = (a & 0x7) ^ 5 ^ 6 ^ c
    //   a >>= 3
    //   print b[0..3]
    //   if a != 0, repeat
    //
    // Resolve in inverse order

    let &expected_iter_output = PROGRAM.last().unwrap();
    for a in 0.. {
        if get_iteration_output(a) == expected_iter_output {
            if let Some(a_found) = part2_rev_iteration(&PROGRAM[..PROGRAM.len() - 1], a) {
                return a_found;
            }
        }
    }

    panic!();
}

fn part2_rev_iteration(program: &[i64], mut a: i64) -> Option<i64> {
    if program.is_empty() {
        return Some(a);
    }

    a <<= 3;
    let &expected_iter_output = program.last().unwrap();
    for a_low_3_bits in 0..=0b111 {
        if get_iteration_output(a | a_low_3_bits) == expected_iter_output {
            if let Some(a_found) =
                part2_rev_iteration(&program[..program.len() - 1], a | a_low_3_bits)
            {
                return Some(a_found);
            }
        }
    }
    None
}

fn get_iteration_output(a: i64) -> i64 {
    let a_3bit = a & 0b111;
    let c = a / 2i64.pow(a_3bit as u32 ^ 5);
    (a ^ 5 ^ 6 ^ c) & 0x7
}

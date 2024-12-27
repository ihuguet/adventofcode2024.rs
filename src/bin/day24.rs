use adventofcode2024 as aoc;
use std::collections::{HashMap, HashSet};

const INPUT_BITS: usize = 45;

#[derive(Clone)]
struct Op {
    in1: String,
    in2: String,
    out: String,
    op: String,
}

// How to solve part 2:
// 1. Generate a visual graph with the gates:
//    1. Run this binary with `./day24 --gen-graph > gates.dot`
//    2. Convert to png with `dot -Tpng gates.dot > gates.png`
// 2. Find what bits are being calculated wrong with `./day24 --print-errors`
// 3. Inspect the graph and see what wires are exchanged, searching in the
//    areas of the input and output bits found in step 2. The same design of
//    gates is used for each input/output gates stage.
fn main() {
    let (wires, mut operations) = parse_input();

    let args: Vec<_> = std::env::args().collect();
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "--print-errors" => {
                let ok = print_circuit_errors(&operations);
                println!("Errors: {}", if ok { "no" } else { "yes " });
            }
            "--gen-graph" => print_dot_graph(&operations),
            arg => panic!("Unkown argument '{arg}'"),
        }
        return;
    }

    // Part 1
    let wires_result = solve(wires.clone(), operations.clone());
    let result1 = get_wires_num(&wires_result, "z");
    println!("Part 1: result={result1}");

    // Part 2
    let exchange_pairs = [
        ("swt", "z07"),
        ("pqc", "z13"),
        ("wsv", "rjm"),
        ("bgs", "z31"),
    ];
    for op in &mut operations {
        for pair in &exchange_pairs {
            if op.out == pair.0 {
                op.out = pair.1.to_string();
            } else if op.out == pair.1 {
                op.out = pair.0.to_string();
            }
        }
    }
    if !print_circuit_errors(&operations) {
        panic!("Part 2 ERROR");
    }

    let mut exchange_wires: Vec<_> = exchange_pairs
        .into_iter()
        .flat_map(|pair| [pair.0, pair.1])
        .collect();
    exchange_wires.sort();
    println!("Pair 2: wires={}", exchange_wires.join(","));
}

fn solve(mut wires: HashMap<String, bool>, mut operations: Vec<Op>) -> HashMap<String, bool> {
    while !operations.is_empty() {
        let mut i = 0;

        while i < operations.len() {
            let op = &operations[i];

            if !wires.contains_key(&op.in1) || !wires.contains_key(&op.in2) {
                i += 1;
                continue;
            }

            let v1 = wires[&op.in1];
            let v2 = wires[&op.in2];
            let vo = match op.op.as_str() {
                "AND" => v1 && v2,
                "OR" => v1 || v2,
                "XOR" => v1 != v2,
                _ => panic!(),
            };

            let op = operations.remove(i);
            wires.insert(op.out, vo);
        }
    }

    wires
}

fn print_circuit_errors(operations: &[Op]) -> bool {
    let mut ok = true;

    'outer: for i in 0..INPUT_BITS {
        for [x, y] in [[0, 0], [0, 1], [1, 0], [1, 1]] {
            for carry in [0, 1] {
                let mut x_num = (x as u64) << i;
                let mut y_num = (y as u64) << i;
                if i > 0 && carry == 1 {
                    x_num |= 1u64 << (i - 1);
                    y_num |= 1u64 << (i - 1);
                } else if i == 0 && carry == 1 {
                    continue;
                }

                let wires = prepare_input_wires(x_num, y_num);
                let wires = solve(wires, operations.to_vec());
                let result = get_wires_num(&wires, "z");
                let expect = if carry == 0 {
                    (x + y) << i
                } else {
                    (x + y + 1) << i
                };

                if result != expect {
                    println!(
                        "Error adding bit {i}: expected {:1b}+{:1b}{}={:02b}, got {:02b}",
                        x,
                        y,
                        if carry == 1 { "+carry" } else { "" },
                        expect >> i,
                        result >> i
                    );
                    ok = false;
                    continue 'outer;
                }
            }
        }
    }

    ok
}

fn print_dot_graph(operations: &[Op]) {
    println!("digraph {{");

    let mut graph_input_to_nodes: HashMap<String, HashSet<String>> = HashMap::new();
    for op in operations {
        let name = format!("{}{}{}{}", &op.in1, &op.op, &op.in2, &op.out);
        println!(
            "    {name}[label=\"{} {} {} = {}\"];",
            &op.in1, &op.op, &op.in2, &op.out
        );
        graph_input_to_nodes
            .entry(op.in1.clone())
            .or_default()
            .insert(name.clone());
        graph_input_to_nodes
            .entry(op.in2.clone())
            .or_default()
            .insert(name);
    }

    for op in operations {
        let name_from = format!("{}{}{}{}", &op.in1, &op.op, &op.in2, &op.out);
        if let Some(dsts) = graph_input_to_nodes.get(&op.out) {
            for name_dst in dsts {
                println!("    {name_from} -> {name_dst};");
            }
        }
    }

    println!("}}");
}

fn get_wires_num(wires: &HashMap<String, bool>, prefix: &str) -> u64 {
    let mut result = 0u64;
    for (wire, val) in wires {
        if let Some(num) = wire.strip_prefix(prefix) {
            let bit: u32 = num.parse().unwrap();
            result |= (*val as u64) << bit;
        }
    }
    result
}

fn prepare_input_wires(x: u64, y: u64) -> HashMap<String, bool> {
    let mut wires = HashMap::new();
    for i in 0..INPUT_BITS {
        let x_str = format!("x{:02}", i);
        let x_val = ((x >> i) & 0x1) != 0;
        let y_str = format!("y{:02}", i);
        let y_val = ((y >> i) & 0x1) != 0;
        wires.insert(x_str, x_val);
        wires.insert(y_str, y_val);
    }
    wires
}

fn parse_input() -> (HashMap<String, bool>, Vec<Op>) {
    let mut lines = aoc::input::read_lines("day24");

    let mut wires = HashMap::new();
    for line in lines.by_ref().take_while(|line| line != "") {
        let mut split = line.split(": ");
        let wire = split.next().unwrap().to_string();
        let val = split.next().unwrap() == "1";
        wires.insert(wire, val);
    }

    let mut operations = Vec::new();
    for line in lines {
        let mut split = line.split(" -> ");
        let operation = split.next().unwrap();
        let out = split.next().unwrap().to_string();
        let mut split = operation.split(" ");
        let in1 = split.next().unwrap().to_string();
        let op = split.next().unwrap().to_string();
        let in2 = split.next().unwrap().to_string();
        operations.push(Op { in1, in2, out, op });
    }

    (wires, operations)
}

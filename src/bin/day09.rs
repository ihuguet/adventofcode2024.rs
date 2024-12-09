use std::fs;

#[derive(Clone, Copy)]
struct DiskSpace {
    pos: usize,
    len: usize,
}

fn main() {
    let blocks = parse_input();

    let sum = part1(blocks.clone());
    println!("Part 1: checksum={sum}");

    let sum = part2(blocks);
    println!("Part 2: checksum={sum}");
}

fn part1(mut blocks: Vec<Option<u64>>) -> u64 {
    let mut src = blocks.iter().rposition(|v| v.is_some()).unwrap();
    let mut dst = blocks.iter().position(|v| v.is_none()).unwrap();

    while dst < src {
        blocks[dst] = blocks[src];
        blocks[src] = None;

        dst += 1 + blocks[dst + 1..].iter().position(|v| v.is_none()).unwrap();
        src = blocks[..src].iter().rposition(|v| v.is_some()).unwrap();
    }

    blocks
        .iter()
        .take_while(|v| v.is_some())
        .enumerate()
        .map(|(idx, val)| idx as u64 * val.unwrap())
        .sum()
}

fn part2(mut blocks: Vec<Option<u64>>) -> u64 {
    let mut src = find_used_space_from_right(&blocks, blocks.len()).unwrap();

    while src.pos > 0 {
        let mut dst = find_empty_space_from_left(&blocks, 0).unwrap();
        while dst.pos < src.pos {
            if dst.len >= src.len {
                move_blocks(&mut blocks, src, dst);
                break;
            }
            dst = find_empty_space_from_left(&blocks, dst.pos + dst.len).unwrap();
        }
        src = find_used_space_from_right(&blocks, src.pos).unwrap();
    }

    blocks
        .iter()
        .enumerate()
        .map(|(idx, val)| idx as u64 * val.unwrap_or(0))
        .sum()
}

fn find_empty_space_from_left(blocks: &[Option<u64>], start: usize) -> Option<DiskSpace> {
    if let Some(pos) = blocks[start..].iter().position(|v| v.is_none()) {
        let pos = pos + start;
        let len = blocks[pos..]
            .iter()
            .position(|v| v.is_some())
            .unwrap_or(blocks.len() - pos);
        return Some(DiskSpace { pos, len });
    }
    None
}

fn find_used_space_from_right(blocks: &[Option<u64>], end: usize) -> Option<DiskSpace> {
    if let Some(end) = blocks[..end].iter().rposition(|v| v.is_some()) {
        if let Some(start) = blocks[..end].iter().rposition(|v| *v != blocks[end]) {
            return Some(DiskSpace {
                pos: start + 1,
                len: end - start,
            });
        } else {
            return Some(DiskSpace {
                pos: 0,
                len: end + 1,
            });
        }
    }
    None
}

fn move_blocks(blocks: &mut [Option<u64>], src: DiskSpace, dst: DiskSpace) {
    for i in 0..src.len {
        blocks[dst.pos + i] = blocks[src.pos + i];
        blocks[src.pos + i] = None;
    }
}

fn parse_input() -> Vec<Option<u64>> {
    let input: Vec<_> = fs::read_to_string("input/day09.txt")
        .unwrap()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let mut id = 0;
    let mut blocks = Vec::new();
    for chunk in input.chunks(2) {
        let used_blocks = chunk[0];
        blocks.extend(std::iter::repeat_n(Some(id), used_blocks));
        if chunk.len() == 2 {
            let free_blocks = chunk[1];
            blocks.extend(std::iter::repeat_n(None, free_blocks));
        }
        id += 1;
    }

    blocks
}

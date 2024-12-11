use std::collections::HashMap;

const INPUT: [u64; 8] = [27, 10647, 103, 9, 0, 5524, 4594227, 902936];

fn main() {
    let mut nums: HashMap<u64, usize> = INPUT.iter().map(|&n| (n, 1)).collect();
    let mut nums2: HashMap<u64, usize> = HashMap::new();
    let (mut src, mut dst) = (&mut nums, &mut nums2);

    for iteration in 0..75 {
        if iteration == 25 {
            println!("Part 1: stones num={}", src.values().sum::<usize>());
        }

        // Do the operation for each unique number that we have. The same result
        // is valid for all the numbers with the same value. Count holds the
        // number of times that we have the same number repeated.
        for (&n, &count) in src.iter() {
            let result_nums: &[u64] = match n {
                0 => &[1],
                n => match get_split_divisor(n) {
                    Some(div) => &[n / div, n % div],
                    None => &[n * 2024],
                },
            };

            for &n in result_nums {
                *dst.entry(n).or_default() += count;
            }
        }

        (src, dst) = (dst, src);
        dst.clear();
    }

    println!("Part 2: stones num={}", src.values().sum::<usize>());
}

fn get_split_divisor(n: u64) -> Option<u64> {
    let mut digits = 1;
    let mut cmp = 10;
    while n >= cmp {
        cmp *= 10;
        digits += 1;
    }
    if digits % 2 == 0 {
        Some(10u64.pow(digits / 2))
    } else {
        None
    }
}

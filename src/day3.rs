use aoc_runner_derive::{aoc};

#[aoc(day3, part1)]
fn part1(input: &[u8]) -> i64 {
    input.split(|b| *b == b'\n').map(|bytes| {
        let mut set_f = 0_u64;
        let mut set_b = 0_u64;
        let len = bytes.len();

        for index in 0..(len / 2) {
            let front = char_to_prio(bytes[index]);
            let back = char_to_prio(bytes[len - index - 1]);

            set_f |= 1 << front;
            set_b |= 1 << back;

            if set_b & set_f != 0 {
                return log2(set_b & set_f) as u32;
            }
        }

        0
    }).sum::<u32>() as i64
}

#[inline]
fn char_to_prio(char: u8) -> u8 {
    if char <= 0x5A {
        char - 0x40 + 26
    } else {
        char - 0x60
    }
}

#[aoc(day3, part2)]
fn part2(input: &[u8]) -> i64 {
    input.split(|b| *b == b'\n').fold((0_u16, 0, [0_u64, 0_u64, 0_u64]), |(acc, it, mut sets), bytes| {
        let mut lset = 0_u64;

        for index in 0..bytes.len() {
            let front = char_to_prio(bytes[index]);

            lset |= 1 << front;
        }

        sets[it % 3] = lset;

        if it % 3 == 2 {
            let comm = log2(sets[0] & sets[1] & sets[2]);

            (acc + comm as u16, it + 1, sets)
        } else {
            (acc, it + 1, sets)
        }
    }).0 as i64
}

#[inline]
fn log2(val: u64) -> u8 {
    val.trailing_zeros() as u8
}

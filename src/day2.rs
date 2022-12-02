use aoc_runner_derive::{aoc};
use std::mem::transmute;

const X: u8 = b'X' - 1;
const A: u8 = b'A';

const MASK: u32 = 0x00570041;

#[aoc(day2, part1)]
fn part1(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    unsafe {
        transmute::<&[u8], &[u32]>(&input[0..input.len() / 4]).iter().map(|chr| {
            let chrs    = transmute::<u32, [u16; 2]>((*chr & 0x00FF00FF) - MASK);

            let prev    = chrs[0];
            let curr    = chrs[1];

            let outcome = (prev * 2 + curr) % 3;

            (outcome * 3) + curr
        }).sum::<u16>() as i64
    }
}

#[aoc(day2, part2)]
fn part2(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    input.as_slice().iter().fold((0_i64, [0,0,0_u8], 0), |(val, mut s, ctr), x| match x {
        b'\n' => {
            (val + match s {
                [65, 32, 88] => 3, // AX
                [65, 32, 89] => 4, // AY
                [65, 32, 90] => 8, // AZ
                [66, 32, 88] => 1, // BX
                [66, 32, 89] => 5, // BY
                [66, 32, 90] => 9, // BZ
                [67, 32, 88] => 2, // CX
                [67, 32, 89] => 6, // CY
                [67, 32, 90] => 7, // CZ
                _ => 0
            }, [0,0,0], 0)
        }
        x => {
            s[ctr] = *x;
            (val, s, ctr + 1)
        }
    }
    ).0
}

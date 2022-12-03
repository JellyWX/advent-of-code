use aoc_runner_derive::{aoc};
use std::mem::transmute;

#[aoc(day2, part1)]
fn part1(input_: &[u8]) -> i64 {
    let input = [input_, &[b'\n']].concat();

    unsafe {
        let res = transmute::<&[u8], &[u32]>(&input[0..input.len() / 4]).iter().fold((0, 0), |(a, b), chr| {
            let chrs = transmute::<u32, [u16; 2]>(*chr - 0x0A572041);

            let outcome = (chrs[0] * 2 + chrs[1]) % 3;

            (a + outcome, b + chrs[1])
        });

        (res.0 * 3 + res.1) as i64
    }
}

#[aoc(day2, part2)]
fn part2(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    unsafe {
        let res = transmute::<&[u8], &[u32]>(&input[0..input.len() / 4]).iter().fold((0, 0), |(a, b), chr| {
            let chrs   = transmute::<u32, [u16; 2]>(*chr - 0x0A582039);

            let played = (chrs[0] + chrs[1]) % 3;

            (a + chrs[1], b + played)
        });

        (res.0 * 3 + res.1 + (input.len() as u16) / 4) as i64
    }
}

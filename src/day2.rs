use aoc_runner_derive::{aoc};
use std::mem::transmute;

#[aoc(day2, part1)]
fn part1(input_: &[u8]) -> i64 {
    let input = [input_, &[b'\n']].concat();

    unsafe {
        let res = transmute::<&[u8], &[u32]>(&input[0..input.len() / 4]).iter().fold((0, 0), |(a, b), chr| {
            let chrs    = transmute::<u32, [u16; 2]>((*chr & 0x00FF00FF) - 0x00570041);

            let outcome = (chrs[0] * 2 + chrs[1]) % 3;
            
            (a + outcome, b + chrs[1])
        });

        (res.0 as i64) * 3 + res.1 as i64
    }
}

#[aoc(day2, part2)]
fn part2(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    unsafe {
        transmute::<&[u8], &[u32]>(&input[0..input.len() / 4]).iter().map(|chr| {
            let chrs   = transmute::<u32, [u16; 2]>((*chr & 0x00FF00FF) - 0x00580039);

            let played = (chrs[0] + chrs[1]) % 3;

            chrs[1] * 3 + played + 1
        }).sum::<u16>() as i64
    }
}

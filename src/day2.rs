use aoc_runner_derive::{aoc};
use std::mem::transmute;

const X: i64 = b'X' as i64;
const A: i64 = b'A' as i64;

#[aoc(day2, part1)]
fn part1(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    unsafe {
        transmute::<&[u8], &[u16]>(&input[0..input.len() / 2]).iter().fold((0_i64, 0, false), |(val, prev, ctr), chr| match ctr {
            false => {
                (val, transmute::<u16, [u8; 2]>(*chr)[0] as i64, true)
            }
            true => {
                let chr     = transmute::<u16, [u8; 2]>(*chr)[0] as i64;
                let bonus   = chr - X + 1;
                let outcome = ((prev - A) * 2 + bonus) % 3;
                
                (val + (outcome * 3) + bonus, 0, false)
            }
        }
        ).0
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

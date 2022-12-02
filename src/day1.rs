use aoc_runner_derive::{aoc};
use std::mem::transmute;

const ZERO: u8 = 48;

#[aoc(day1, part1)]
fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .fold(0, |acc, i| i
             .split("\n")
             .map(|x| atoi(x.as_bytes()))
             .reduce(|a, b| a + b)
             .unwrap()
             .max(acc)
        ) as i64
}

fn atoi(s: &[u8]) -> u32 {
    s.iter()
        .fold(0, |acc, v| {
            acc * 10 + (v - ZERO) as u32
        })
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i64 {
    let (a, b, c) = input
        .split("\n\n")
        .map(|i| i
             .split("\n")
             .map(|x| atoi(x.as_bytes()))
             .reduce(|a, b| a + b)
             .unwrap()
        )
        .fold((0, 0, 0), |(a, b, c), x| {
            if x < c {
                (a, b, c)
            } else {
                if x > a {
                    (x, a, b)
                } else if x > b {
                    (a, x, b)
                } else {
                    (a, b, x)
                }
            }
        });

    (a + b + c) as i64
}



use aoc_runner_derive::{aoc};

#[aoc(day2, part1)]
fn part1(input_: &[u8]) -> i64 {
    let input = [input_, b"\n"].concat();

    input.as_slice().iter().fold((0_i64, [0,0,0_u8], 0), |(val, mut s, ctr), x| match x {
        b'\n' => {
            (val + match s {
                [65, 32, 88] => 4,
                [65, 32, 89] => 8,
                [65, 32, 90] => 3,
                [66, 32, 88] => 1,
                [66, 32, 89] => 5,
                [66, 32, 90] => 9,
                [67, 32, 88] => 7,
                [67, 32, 89] => 2,
                [67, 32, 90] => 6,
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

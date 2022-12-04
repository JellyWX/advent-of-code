use aoc_runner_derive::{aoc};

#[aoc(day4, part1)]
fn part1(input: &[u8]) -> i64 {
    input.split(|b| *b == b'\n').filter(|bytes| {
        let mut vals = [0_u16, 0_u16, 0_u16, 0_u16];
        let mut i = 0;

        while bytes[i] != b'-' {
            vals[0] = (vals[0] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while bytes[i] != b',' {
            vals[1] = (vals[1] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while bytes[i] != b'-' {
            vals[2] = (vals[2] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while i < bytes.len() {
            vals[3] = (vals[3] << 8) + bytes[i] as u16;
            i += 1;
        }

        (vals[0] <= vals[2] && vals[1] >= vals[3])
            || (vals[0] >= vals[2] && vals[1] <= vals[3])
    }).count() as i64
}

#[aoc(day4, part2)]
fn part2(input: &[u8]) -> i64 {
    input.split(|b| *b == b'\n').filter(|bytes| {
        let mut vals = [0_u16, 0_u16, 0_u16, 0_u16];
        let mut i = 0;

        while bytes[i] != b'-' {
            vals[0] = (vals[0] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while bytes[i] != b',' {
            vals[1] = (vals[1] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while bytes[i] != b'-' {
            vals[2] = (vals[2] << 8) + bytes[i] as u16;
            i += 1;
        }
        i += 1;
        while i < bytes.len() {
            vals[3] = (vals[3] << 8) + bytes[i] as u16;
            i += 1;
        }

        vals[1] >= vals[2] && vals[0] <= vals[3]
    }).count() as i64
}

use aoc_runner_derive::{aoc};

const ZERO: u8 = 48;

#[aoc(day4, part1)]
fn part1(input: &[u8]) -> i64 {
    input.split(|b| *b == b'\n').filter(|bytes| {
        let mut vals = [0_u8, 0_u8, 0_u8, 0_u8];
        let mut i = 0;

        vals[0] = bytes[i];
        i += 1;
        if bytes[i] >= ZERO {
            vals[0] = (vals[0] << 4) + (bytes[i] & 0b00001111);
            i += 1;
        } else {
            vals[0] &= 0b00001111;
        }
        i += 1;

        vals[1] = bytes[i];
        i += 1;
        if bytes[i] >= ZERO {
            vals[1] = (vals[1] << 4) + (bytes[i] & 0b00001111);
            i += 1;
        } else {
            vals[1] &= 0b00001111;
        }
        i += 1;

        vals[2] = bytes[i];
        i += 1;
        if bytes[i] >= ZERO {
            vals[2] = (vals[2] << 4) + (bytes[i] & 0b00001111);
            i += 1;
        } else {
            vals[2] &= 0b00001111;
        }
        i += 1;

        vals[3] = bytes[i];
        i += 1;
        if i < bytes.len() {
            vals[3] = (vals[3] << 4) + (bytes[i] & 0b00001111);
        } else {
            vals[3] &= 0b00001111;
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

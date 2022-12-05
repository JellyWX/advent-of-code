use aoc_runner_derive::{aoc};

#[aoc(day5, part1)]
fn part1(input: &[u8]) -> String {
    let initial_rows = input
        .split(|b| *b == b'\n')
        .take_while(|line| line[1] != b'1')
        .count();

    let max_height = initial_rows * 9;

    let mut stacks: [Vec<u8>; 9] = [
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
        Vec::with_capacity(max_height),
    ];

    input
        .array_chunks::<4>()
        .take_while(|chk| chk[1] != b'1')
        .enumerate()
        .for_each(|(count, chk)| {
            if chk[1] != b' ' {
                stacks[count % 9].insert(0, chk[1]);
            }
        });

    input
        .split(|b| *b == b'\n')
        .skip_while(|b| b.len() > 0)
        .filter(|l| l.len() > 0)
        .for_each(|line| {
            let mut amount = line[5] - b'0';
            let mut i = 0;

            if line[6] != b' ' {
                amount = amount * 10 + line[6] - b'0';
                i = 1;
            }

            let pos_1 = (line[12 + i] - b'1') as usize;
            let pos_2 = (line[17 + i] - b'1') as usize;

            for _ in 0..amount {
                let chr = stacks[pos_1].pop().unwrap();
                stacks[pos_2].push(chr);
            }
        });

    let mut s = String::new();

    for mut stack in stacks {
        s.push(stack.pop().unwrap() as char);
    }

    s
}

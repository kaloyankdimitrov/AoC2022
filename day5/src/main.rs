use std::fs;

fn init(starting: &str, crates: &mut [Vec<char>; 9]) {
    starting.lines().rev().skip(1).for_each(|line| {
        line.chars().enumerate().for_each(|(ind, ch)| {
            if ind % 4 == 1 && !ch.is_whitespace() {
                crates[ind / 4].push(ch);
            }
        })
    });
}

fn execute(procedure: &str, crates: &mut [Vec<char>; 9], reverse: bool) {
    procedure.lines().for_each(|line| {
        if let [count, from, to] = line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            // convert to index
            let mut removed = crates[from - 1].split_off(crates[from - 1].len() - count);
            if reverse {
                removed.reverse();
            }
            crates[to - 1].extend_from_slice(&removed[..]);
        }
    });
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (starting, procedure) = input.trim_end().split_once("\n\n").unwrap();
    let mut crates: [Vec<char>; 9] = Default::default();
    init(starting, &mut crates);
    let mut crates2 = crates.clone();
    execute(procedure, &mut crates, true);

    let answer1 = crates
        .iter()
        .map(|c| *c.last().unwrap())
        .collect::<String>();
    println!("Answer 1: {answer1}");

    execute(procedure, &mut crates2, false);
    let answer2 = crates2
        .iter()
        .map(|c| *c.last().unwrap())
        .collect::<String>();
    println!("Answer 2: {answer2}");
}

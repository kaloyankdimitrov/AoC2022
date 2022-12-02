use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut totals = input
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split("\n")
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();
    totals.sort();
    println!("part 1: {}", totals.last().unwrap());
    println!("part 2: {}", totals[totals.len() - 3..].iter().sum::<i32>());
}

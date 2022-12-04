use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let answer1 = input
        .trim()
        .lines()
        .filter(|pair| {
            let sections: Vec<Vec<u32>> = pair
                .split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();
            (sections[0][0] <= sections[1][0] && sections[0][1] >= sections[1][1])
                || (sections[1][0] <= sections[0][0] && sections[1][1] >= sections[0][1])
        })
        .count();
    println!("Answer 1: {answer1}");
    let answer2 = input
        .trim()
        .lines()
        .filter(|pair| {
            let sections: Vec<Vec<u32>> = pair
                .split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();
            (sections[0][0] >= sections[1][0] && sections[0][0] <= sections[1][1])
                || (sections[0][1] >= sections[1][0] && sections[0][1] <= sections[1][1])
                || (sections[1][0] >= sections[0][0] && sections[1][0] <= sections[0][1])
        })
        .count();
    println!("Answer 2: {answer2}");
}

use std::fs;

fn convert_char(ch: char) -> usize {
    let uch = ch as usize;
    if uch >= 'a' as usize {
        return uch - 'a' as usize + 1;
    } else {
        return uch - 'A' as usize + 27;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let answer1: u32 = input
        .trim()
        .lines()
        .map(|line| {
            let sack: Vec<char> = line.chars().collect();
            let mut exists: [bool; 53] = [false; 53];
            let compartments = (&sack[..line.len() / 2], &sack[line.len() / 2..]);
            compartments
                .0
                .iter()
                .for_each(|ch| exists[convert_char(*ch)] = true);
            convert_char(
                *compartments
                    .1
                    .iter()
                    .find(|ch| exists[convert_char(**ch)])
                    .unwrap(),
            ) as u32
        })
        .sum();
    println!("Answer 1: {answer1}");
    let answer2: u32 = input
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let (mut exists1, mut exists2) = ([false; 53], [false; 53]);
            group[0].chars().for_each(|ch| {
                exists1[convert_char(ch)] = true;
            });
            group[1].chars().for_each(|ch| {
                exists2[convert_char(ch)] = true;
            });
            group[2]
                .chars()
                .map(|ch| convert_char(ch))
                .find(|val| exists1[*val] && exists2[*val])
                .unwrap() as u32
        })
        .sum();
    println!("Answer 2: {answer2}");
}

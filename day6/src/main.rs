use std::fs;

fn char_to_index(ch: char) -> u32 {
    1u32 << (ch as u8 - 'a' as u8)
}

fn is_marker(bytes: &[u32]) -> bool {
    let mut set = 0u32;
    for b in bytes.iter() {
        if set & b != 0 {
            return false;
        } else {
            set |= b;
        }
    }
    true
}

fn main() {
    let input: Vec<u32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|ch| char_to_index(ch))
        .collect();
    let answer1 = input
        .windows(4)
        .enumerate()
        .find(|(_, bytes)| is_marker(*bytes))
        .unwrap()
        .0
        + 4;
    println!("Answer 1: {answer1}");
    let answer2 = input
        .windows(14)
        .enumerate()
        .find(|(_, bytes)| is_marker(*bytes))
        .unwrap()
        .0
        + 14;
    println!("Answer 2: {answer2}");
}

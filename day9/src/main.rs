use std::{collections::HashSet, fs};

fn calc_pos(prev: (i32, i32), mut curr: (i32, i32)) -> (i32, i32) {
    if prev.0.abs_diff(curr.0) > 1 || prev.1.abs_diff(curr.1) > 1 {
        if prev.0 > curr.0 {
            curr.0 += 1;
        }
        if prev.0 < curr.0 {
            curr.0 -= 1;
        }
        if prev.1 > curr.1 {
            curr.1 += 1;
        }
        if prev.1 < curr.1 {
            curr.1 -= 1;
        }
    }
    curr
}

fn solve(input: &str, knots: usize) -> usize {
    let mut rope = vec![(0, 0); knots];
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(*rope.last().unwrap());
    for line in input.trim().lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        for _ in 0..steps.parse().unwrap() {
            match dir {
                "L" => rope.first_mut().unwrap().0 -= 1,
                "R" => rope.first_mut().unwrap().0 += 1,
                "U" => rope.first_mut().unwrap().1 -= 1,
                "D" => rope.first_mut().unwrap().1 += 1,
                &_ => {}
            }
            for i in 1..rope.len() {
                rope[i] = calc_pos(rope[i - 1], rope[i])
            }
            set.insert(*rope.last().unwrap());
        }
    }
    set.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer 1: {}", solve(&input, 2));
    println!("Answer 2: {}", solve(&input, 10));
}

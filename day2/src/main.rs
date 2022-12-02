use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let total: usize = input
        .trim()
        .split("\n")
        .map(|round| {
            let (a, b) = (
                round.chars().nth(0).unwrap() as usize - 'A' as usize,
                round.chars().nth(2).unwrap() as usize - 'X' as usize,
            );
            let mut transform = vec![1, 2, 0];
            transform.rotate_right(a);
            transform[b] * 3 + b + 1
        })
        .sum();
    let total2: usize = input
        .trim()
        .split("\n")
        .map(|round| {
            let (a, b) = (
                round.chars().nth(0).unwrap() as usize - 'A' as usize,
                round.chars().nth(2).unwrap() as usize - 'X' as usize,
            );
            let transform = vec![2, 0, 1];
            let c = (a + transform[b]) % 3;
            b * 3 + c + 1
        })
        .sum();
    println!("Result: {total}");
    println!("Result 2: {total2}");
}

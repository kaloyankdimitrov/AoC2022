use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);
            let (mut visible_left, mut visible_right, mut visible_up, mut visible_down) =
                (true, true, true, true);
            for k in 1..usize::max(grid.len(), grid[i].len()) {
                if i >= k {
                    if visible_up {
                        up += 1;
                    }
                    if grid[i - k][j] >= grid[i][j] {
                        visible_up = false;
                    }
                }
                if i + k < grid.len() {
                    if visible_down {
                        down += 1;
                    }
                    if grid[i + k][j] >= grid[i][j] {
                        visible_down = false;
                    }
                }
                if j >= k {
                    if visible_left {
                        left += 1;
                    }
                    if grid[i][j - k] >= grid[i][j] {
                        visible_left = false;
                    }
                }
                if j + k < grid[i].len() {
                    if visible_right {
                        right += 1;
                    }
                    if grid[i][j + k] >= grid[i][j] {
                        visible_right = false;
                    }
                }
            }
            if visible_left || visible_right || visible_up || visible_down {
                ans1 += 1;
            }
            ans2 = ans2.max(left * right * up * down);
        }
    }
    println!("Answer 1: {ans1}");
    println!("Answer 2: {ans2}");
}

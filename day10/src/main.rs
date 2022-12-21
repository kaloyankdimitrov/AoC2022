use std::fs;

struct Clock {
    target: usize,
    sum: i32,
    cycle: usize,
    register: i32,
    screen: [[char; 40]; 6],
}

impl Clock {
    fn new() -> Self {
        Clock {
            target: 20,
            sum: 0,
            cycle: 0,
            register: 1,
            screen: [['.'; 40]; 6],
        }
    }
    fn tick(&mut self) {
        if self.register.abs_diff(self.cycle as i32 % 40) <= 1 {
            self.screen[self.cycle / 40][self.cycle % 40] = '#';
        }
        self.cycle += 1;
        if self.cycle == self.target {
            self.sum += self.register * self.cycle as i32;
            if self.target < 220 {
                self.target += 40;
            }
        }
    }

    fn add_x(&mut self, x: i32) {
        self.register += x;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut clock = Clock::new();
    for line in input.trim().lines() {
        match line {
            "noop" => clock.tick(),
            comm => {
                clock.tick();
                clock.tick();
                let (_, val) = comm.split_once(" ").unwrap();
                clock.add_x(val.parse().unwrap());
            }
        }
    }
    println!("Answer 1: {}", clock.sum);
    println!("Answer 2:");
    for i in 0..clock.screen.len() {
        for j in 0..clock.screen[i].len() {
            print!("{}", clock.screen[i][j]);
        }
        println!();
    }
}

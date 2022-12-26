use std::fs;

enum Operator {
    Num(u128),
    Old,
}

struct Monkey<'a> {
    items: Vec<u128>,
    test: u128,
    if_true: usize,
    if_false: usize,
    op: &'a str,
    op1: Operator,
    op2: Operator,
    inspections: usize,
}

impl Monkey<'_> {
    fn compute_worry(&self, div_by_three: bool, val: u128, modulus: u128) -> u128 {
        let op1 = match self.op1 {
            Operator::Old => val,
            Operator::Num(v) => v,
        };
        let op2 = match self.op2 {
            Operator::Old => val,
            Operator::Num(v) => v,
        };
        let mut res;
        if self.op == "+" {
            res = op1 + op2;
        } else {
            res = op1 * op2;
        }
        if div_by_three {
            res /= 3;
        } else {
            res %= modulus;
        }
        res
    }
}

fn solve(rounds: u32, div_by_three: bool) -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut modulus = 1u128;
    for monkey in input.trim().split("\n\n") {
        let mut lines = monkey.lines();
        // skip titular line
        lines.next();
        // get starting lines
        let (_, starting_items) = lines.next().unwrap().split_once(": ").unwrap();
        let operation_tokens: Vec<&str> = lines.next().unwrap().split(" ").collect();
        let other_params: Vec<usize> = lines
            .map(|l| l.split(" ").last().unwrap().parse::<usize>().unwrap())
            .collect();
        monkeys.push(Monkey {
            inspections: 0,
            items: starting_items
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect(),
            test: other_params[0] as u128,
            if_true: other_params[1],
            if_false: other_params[2],
            op: operation_tokens[operation_tokens.len() - 2],
            op1: operation_tokens[operation_tokens.len() - 3]
                .parse()
                .map_or_else(|_| Operator::Old, |val| Operator::Num(val)),
            op2: operation_tokens[operation_tokens.len() - 1]
                .parse()
                .map_or_else(|_| Operator::Old, |val| Operator::Num(val)),
        });
        modulus *= other_params[0] as u128;
    }
    // rounds
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].inspections += monkeys[i].items.len();
            for j in 0..monkeys[i].items.len() {
                let computed = monkeys[i].compute_worry(div_by_three, monkeys[i].items[j], modulus);
                let ind = match computed % monkeys[i].test {
                    0 => monkeys[i].if_true,
                    _ => monkeys[i].if_false,
                };
                monkeys[ind].items.push(computed);
            }
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn main() {
    println!("Answer 1: {}", solve(20, true));
    println!("Answer 2: {}", solve(10000, false));
}

use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, fs};

struct Node {
    pub value: Option<u64>,
    parent: Option<Rc<RefCell<Node>>>,
    children: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: Option<u64>, parent: Option<Rc<RefCell<Node>>>) -> Node {
        return Node {
            value,
            children: HashMap::new(),
            parent,
        };
    }
    pub fn calc_value(&mut self) -> u64 {
        self.value = Some(
            self.children
                .iter()
                .map(|(_, node)| {
                    if let Some(value) = node.borrow().value {
                        return value;
                    }
                    node.borrow_mut().calc_value()
                })
                .sum(),
        );
        self.value.unwrap()
    }
}

fn find_answer1(node: Rc<RefCell<Node>>) -> u64 {
    let mut ans: u64 = 0;
    let node_borrow = node.borrow();
    ans += node_borrow
        .children
        .iter()
        .map(|(_, v)| find_answer1(Rc::clone(v)))
        .sum::<u64>();
    if node_borrow.children.len() != 0 && node_borrow.value.unwrap() <= 100000 {
        ans += node_borrow.value.unwrap();
    }
    ans
}

fn find_answer2(node: Rc<RefCell<Node>>, needed: u64) -> u64 {
    let node_borrow = node.borrow();
    let mut ans = node_borrow
        .children
        .iter()
        .map(|(_, v)| find_answer2(Rc::clone(v), needed))
        .min()
        .unwrap_or(u64::MAX);
    // not file
    if node_borrow.children.len() != 0 && node_borrow.value.unwrap() >= needed {
        ans = ans.min(node_borrow.value.unwrap());
    }
    ans
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let root = Rc::new(RefCell::new(Node::new(None, None)));
    let mut curr = Rc::clone(&root);
    input.trim().lines().for_each(|line| {
        let mut segments = line.split(" ").peekable();
        if *segments.peek().unwrap() == "$" {
            if segments.nth(1).unwrap() == "cd" {
                let dir = segments.next().unwrap();
                if dir == "/" {
                    curr = Rc::clone(&root);
                } else if dir == ".." {
                    let curr_clone = Rc::clone(&curr);
                    curr = Rc::clone(curr_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let curr_clone = Rc::clone(&curr);
                    curr = Rc::clone(curr_clone.borrow().children.get(dir).as_ref().unwrap());
                }
            }
        } else {
            let item = line.split_once(" ").unwrap();
            let child = Rc::new(RefCell::new(Node::new(None, Some(Rc::clone(&curr)))));
            // file
            if item.0 != "dir" {
                child.borrow_mut().value = Some(item.0.parse().unwrap());
            }
            curr.borrow_mut()
                .children
                .insert(item.1.to_string(), Rc::clone(&child));
        }
    });
    root.borrow_mut().calc_value();
    let answer1 = find_answer1(Rc::clone(&root));
    println!("Answer 1: {answer1}");
    let answer2 = find_answer2(
        Rc::clone(&root),
        root.borrow().value.unwrap() - (70000000 - 30000000),
    );
    println!("Answer 2: {answer2}");
}

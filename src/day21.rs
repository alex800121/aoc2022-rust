use std::rc::Rc;
use std::collections::btree_map::BTreeMap;

use project_root::get_project_root;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Leaf {
    Var(Rc<str>),
    Num(isize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Add,
    Mul,
    Div,
    Sub,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tree<T> {
    Node(T),
    Branch {
        op: Op,
        lhs: Box<Tree<T>>,
        rhs: Box<Tree<T>>
    }
}

type TreeMap = BTreeMap<Rc<str>, Result<isize, (Op, Rc<str>, Rc<str>)>>;
fn parse_input(input: impl Iterator<Item = String>) -> TreeMap {
    let mut input_map: TreeMap = BTreeMap::new();
    for i in input {
        let mut i = i.split(' ');
        match (i.next(), i.next(), i.next(), i.next()) {
            (Some(name), Some(value), None, _) => {
                let value = value.parse::<isize>().unwrap();
                input_map.insert(Rc::from(name.trim_end_matches(':')), Ok(value));
            },
            (Some(name), Some(lhs), Some(op), Some(rhs)) => {
                let op = match op {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    "-" => Op::Sub,
                    _ => panic!("{}", op)
                };
                input_map.insert(Rc::from(name.trim_end_matches(':')), Err((op, Rc::from(lhs), Rc::from(rhs))));
            }
            _ => {
                continue;
            }
        }
    }
    input_map
}

fn build_tree_from(input: &TreeMap, root: Rc<str>) -> Tree<isize> {
    let mut stack: Vec<(Op, Result<Tree<isize>, Rc<str>>)> = Vec::new();
    let mut root = root;
    'outer: loop {
        let next_tree = input.get(&root).unwrap();
        match next_tree {
            Ok(i) => {
                let mut current_tree = Tree::Node(*i);
                'inner: loop {
                    match stack.pop() {
                        None => {
                            break 'outer current_tree;
                        },
                        Some((op, Ok(lhs))) => {
                            current_tree = Tree::Branch { op, lhs: Box::from(lhs), rhs: Box::from(current_tree) };
                            continue 'inner;
                        },
                        Some((op, Err(rhs))) => {
                            stack.push((op, Ok(current_tree)));
                            root = rhs;
                            continue 'outer;
                        }
                    }
                }
            },
            Err((op, lhs, rhs)) => {
                stack.push((*op, Err(rhs.clone())));
                root = lhs.clone();
                continue 'outer;
            }
        }
    }
}

pub fn run(input: usize) {
    // let mut input = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input))
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let input = input.trim().lines().map(String::from);
    let input = parse_input(input);
    let tree = build_tree_from(&input, Rc::from("root"));
}

use std::rc::Rc;
use std::collections::btree_map::BTreeMap;

use project_root::get_project_root;

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
                let name = Rc::from(name.trim_end_matches(':'));
                input_map.insert(name, Ok(value));
            },
            (Some(name), Some(lhs), Some(op), Some(rhs)) => {
                let op = match op {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    "-" => Op::Sub,
                    _ => panic!("{}", op)
                };
                let name = Rc::from(name.trim_end_matches(':'));
                let lhs = Rc::from(lhs);
                let rhs = Rc::from(rhs);
                input_map.insert(name, Err((op, lhs, rhs)));
            }
            _ => {
                continue;
            }
        }
    }
    input_map
}

fn build_tree_from<T: Copy>(input: impl Fn(Rc<str>) -> Result<T, (Op, Rc<str>, Rc<str>)>, root: Rc<str>) -> Tree<T> {
    type Stack<T> = Vec<(Op, Result<Tree<T>, Rc<str>>)>;
    let mut stack: Stack<T> = Vec::new();
    let mut next_tree = input(root);
    'outer: loop {
        match next_tree {
            Ok(i) => {
                let mut current_tree = Tree::Node(i);
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
                            next_tree = input(rhs);
                            continue 'outer;
                        }
                    }
                }
            },
            Err((op, lhs, rhs)) => {
                stack.push((op, Err(rhs)));
                next_tree = input(lhs);
                continue 'outer;
            }
        }
    }
}

fn calc_tree<T>(mut tree: Tree<T>, func: impl Fn(Op, T, T) -> T) -> T {
    use Tree::*;
    type Stack<T> = Vec<(Op, Result<T, Tree<T>>)>;
    let mut stack: Stack<T> = Vec::new();
    'outer: loop {
        match tree {
            Branch { op, lhs, rhs } => {
                stack.push((op, Err(*rhs)));
                tree = *lhs;
                continue 'outer;
            },
            Node(mut i) => {
                'inner: loop {
                    match stack.pop() {
                        None => {
                            break 'outer i;
                        },
                        Some((op, Ok(j))) => {
                            i = func(op, j, i);
                            continue 'inner;
                        }
                        Some((op, Err(t))) => {
                            tree = t;
                            stack.push((op, Ok(i)));
                            continue 'outer;
                        },
                    }
                }
            },
        }
    }
}

fn reduce_tree(mut tree: Tree<Option<isize>>) -> Option<isize> {
    use {Op::*, Tree::*};
    type Stack = Vec<(Op, Result<isize, Tree<Option<isize>>>)>;
    let mut stack: Stack = Vec::new();
    'outer: loop {
        match tree {
            Branch {op, lhs, rhs} => {
                stack.push((op, Err(*rhs)));
                tree = *lhs;
                continue 'outer;
            },
            Node(Some(i)) => {
                let mut current_tree = i;
                'inner: loop {
                    match stack.pop() {
                        None => {
                            return None;
                        },
                        Some((op, Ok(j))) => {
                            current_tree = calc(op, j, current_tree);
                            continue 'inner;
                        },
                        Some((op, Err(t))) => {
                            stack.push((op, Ok(current_tree)));
                            tree = t;
                            continue 'outer;
                        }
                    }
                }
            },
            Node(None) => {
                break 'outer;
            }
        }
    };
    let mut solution = Some(0);
    for (op, current_tree) in stack {
        match op {
            Add => {
                let n = current_tree.map_or_else(|e| calc_tree(e, calc_option), Some);
                solution = calc_option(Sub, solution, n);
            }
            Mul => {
                let n = current_tree.map_or_else(|e| calc_tree(e, calc_option), Some);
                solution = calc_option(Div, solution, n);
            },
            Div => {
                match current_tree {
                    Ok(i) => {
                        solution = calc_option(Div, Some(i), solution);
                    },
                    Err(t) => {
                        let n = calc_tree(t, calc_option);
                        solution = calc_option(Mul, solution, n);
                    }
                }
            },
            Sub => {
                match current_tree {
                    Ok(i) => {
                        solution = calc_option(Sub, Some(i), solution);
                    },
                    Err(t) => {
                        let n = calc_tree(t, calc_option);
                        solution = calc_option(Add, solution, n);
                    }
                }
            },
        }
    }
    solution
}

fn calc(op: Op, lhs: isize, rhs: isize) -> isize {
    use Op::*;
    match op {
        Add => lhs + rhs,
        Mul => lhs * rhs,
        Div => lhs / rhs,
        Sub => lhs - rhs,
    }
}

fn calc_option(op: Op, lhs: Option<isize>, rhs: Option<isize>) -> Option<isize> {
    use Op::*;
    Some(match op {
        Add => lhs? + rhs?,
        Mul => lhs? * rhs?, 
        Div => lhs? / rhs?,
        Sub => lhs? - rhs?,
    })
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let input = input.trim().lines().map(String::from);
    let input = parse_input(input);
    let tree = build_tree_from(|x| {
        input.get(&x).unwrap().clone()
    }, Rc::from("root"));
    println!("day21a: {}", calc_tree(tree, calc));
    let mut tree = build_tree_from(|x| {
        if x.as_ref() == "humn" {
            Ok(None)
        } else {
            match input.get(&x).unwrap() {
                Ok(i) => Ok(Some(*i)),
                Err(i) => Err(i.clone())
            }
        }
    }, Rc::from("root"));
    if let Tree::Branch { op, .. } = &mut tree {
        *op = Op::Sub;
    }
    println!("day21b: {}", reduce_tree(tree).unwrap());
}

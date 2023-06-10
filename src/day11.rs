use project_root::get_project_root;
use std::collections::BTreeMap;
use nom::{IResult, bytes::complete, sequence, character};
use num;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation<(), usize>,
    test: Test,
}

#[derive(Debug, Clone)]
struct Test {
    divisible: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
enum Operation<K, T> {
    Const(T),
    Var(K),
    BinOp(BinOp, Box<Operation<K, T>>, Box<Operation<K, T>>),
}

#[derive(Debug, Clone)]
enum BinOp {
    Add,
    Mul,
}

fn parse_operation(input: &str) -> IResult<&str, Operation<(), usize>> {
    let (input, x) = sequence::preceded(complete::tag("Operation: new = "), complete::take_till(|x: char| x.is_whitespace()))(input)?;
    let x = match x {
        "old" => Operation::Var(()),
        _ => Operation::Const(x.parse::<usize>().unwrap()),
    };
    let (input, y) = sequence::preceded(character::complete::multispace0, complete::take_till(|x: char| x.is_whitespace()))(input)?;
    let y = match y {
        "*" => BinOp::Mul,
        _ => BinOp::Add,
    };
    let (input, _) = character::complete::multispace0(input)?;
    let (input, z) = complete::take_till(|x: char| x.is_whitespace())(input)?;
    let z = match z {
        "old" => Operation::Var(()),
        _ => Operation::Const(z.parse::<usize>().unwrap()),
    };
    let (input, _) = character::complete::multispace0(input)?;
    Ok((input, Operation::BinOp(y, Box::new(x), Box::new(z))))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = complete::tag("Test: divisible by ")(input)?;
    let (input, divisible) = character::complete::not_line_ending(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    let divisible: usize = divisible.parse().unwrap();
    let (input, _) = complete::tag("If true: throw to monkey ")(input)?;
    let (input, if_true) = character::complete::not_line_ending(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    let if_true: usize = if_true.parse().unwrap();
    let (input, _) = complete::tag("If false: throw to monkey ")(input)?;
    let (input, if_false) = character::complete::not_line_ending(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    let if_false: usize = if_false.parse().unwrap();
    Ok((input, Test { divisible, if_true, if_false, }))
}

fn parse_monkey(input: &str) -> IResult<&str, (usize, Monkey)> {
    let (input, _) = complete::tag("Monkey ")(input)?;
    let (input, n) = complete::take_till(|x| x == ':')(input)?;
    let n = n.parse::<usize>().unwrap();
    let (input, _) = character::complete::not_line_ending(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    let (input, _) = complete::tag("Starting items: ")(input)?;
    let (input, items) = character::complete::not_line_ending(input)?;
    let items = items.split(", ").map(|x| x.parse::<usize>().unwrap()).collect();
    let (input, _) = character::complete::multispace0(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    let (input, test) = parse_test(input)?;
    let (input, _) = character::complete::multispace0(input)?;
    Ok((input, (n, Monkey {
        items,
        operation,
        test,
    })))
}

fn interpret_op(operation: &Operation<(), usize>, input: usize) -> usize {
    match operation {
        Operation::Const(n) => *n,
        Operation::Var(()) => input,
        Operation::BinOp(BinOp::Add, a, b) => {
            interpret_op(a.as_ref(), input) + interpret_op(b.as_ref(), input)
        },
        Operation::BinOp(BinOp::Mul, a, b) => {
            interpret_op(a.as_ref(), input) * interpret_op(b.as_ref(), input)
        }
    }
}

fn interpret_test(test: &Test, input: usize) -> usize {
    if input % test.divisible == 0 {
        test.if_true
    } else {
        test.if_false
    }
}

fn round1(monkeys: &mut BTreeMap<usize, Monkey>, acc: &mut BTreeMap<usize, usize>, f: impl Fn(usize) -> usize) {
    let keys: Vec<_> = monkeys.keys().cloned().collect();
    for k in keys {
        let mut temp = vec![];
        std::mem::swap(&mut monkeys.get_mut(&k).unwrap().items, &mut temp);
        for item in temp {
            *acc.get_mut(&k).unwrap() += 1;
            let worry = f(interpret_op(&monkeys.get(&k).unwrap().operation, item));
            let next = interpret_test(&monkeys.get(&k).unwrap().test, worry);
            monkeys.get_mut(&next).unwrap().items.push(worry);
        }
    };
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let mut monkeys: BTreeMap<usize, Monkey> = BTreeMap::from_iter(input.split("\n\n").map(|x| parse_monkey(x).unwrap().1));
    let mut monkeys2 = monkeys.clone();
    let lcm = monkeys.iter().fold(1, |x, y| num::integer::lcm(x, y.1.test.divisible));
    let mut acc = BTreeMap::from_iter(monkeys.keys().cloned().map(|x| (x, 0)));
    (0..20).for_each(|_| round1(&mut monkeys, &mut acc, |x| x / 3));
    let mut acc = acc.values().collect::<Vec<_>>();
    acc.sort();
    acc.reverse();
    println!("day11a: {}", acc[0..2].iter().fold(1, |x, y| x * **y));
    let mut acc = BTreeMap::from_iter(monkeys.keys().cloned().map(|x| (x, 0)));
    (0..10000).for_each(|_| round1(&mut monkeys2, &mut acc, |x| x % lcm));
    let mut acc = acc.values().collect::<Vec<_>>();
    acc.sort();
    acc.reverse();
    println!("day11a: {}", acc[0..2].iter().fold(1, |x, y| x * **y));
}

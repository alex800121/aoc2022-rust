use project_root::get_project_root;
use std::collections::HashMap;
use nom::{IResult, bytes, character::complete::space0};

fn to_crates(s: &str) -> HashMap<usize, Vec<char>> {
    let mut s: Vec<Vec<char>> = s.lines().map(|x| x.chars().collect()).collect();
    s.reverse();
    // let mut s: Vec<Vec<char>> = s.into_iter().filter(|x| x[0].is_ascii_digit()).collect();
    // s.iter_mut().for_each(|x| { x.remove(0); });
    // s
    let mut t = HashMap::new();
    for i in 0..s[0].len() {
        if s[0][i].is_ascii_digit() {
            let mut x = Vec::new();
            let mut n = 0;
            for j in s.iter() {
                if j[i].is_ascii_digit() {
                    n = j[i].to_digit(10).unwrap() as usize;
                } else if !j[i].is_whitespace() {
                    x.push(j[i]);
                } else {}
            }
            t.insert(n, x);
        }
    }
    t
}

fn instructions_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = bytes::complete::tag("move ")(input)?;
    let (input, move_n) = bytes::complete::take_till(|x: char| x.is_whitespace())(input)?;
    let move_n = move_n.parse().unwrap();
    let (input, _) = space0(input)?;
    let (input, _) = bytes::complete::tag("from ")(input)?;
    let (input, from_crate) = bytes::complete::take_till(|x: char| x.is_whitespace())(input)?;
    let from_crate = from_crate.parse().unwrap();
    let (input, _) = space0(input)?;
    let (input, _) = bytes::complete::tag("to ")(input)?;
    let (input, to_crate) = bytes::complete::take_till(|x: char| x.is_whitespace())(input)?;
    let to_crate = to_crate.parse().unwrap();
    let (input, _) = space0(input)?;
    Ok((input, Instruction {
        move_n,from_crate,to_crate,
    }))
}

#[derive(Debug)]
struct Instruction {
    move_n: usize,
    from_crate: usize,
    to_crate: usize,
}

fn interpret(ins: &Instruction, x: &mut HashMap<usize, Vec<char>>) {
    let from_crate = x.get_mut(&ins.from_crate).unwrap();
    let n = from_crate.len() - ins.move_n;
    let mut sli: Vec<char> = from_crate.splice(n..from_crate.len(), []).collect();
    sli.reverse();
    x.get_mut(&ins.to_crate).unwrap().append(&mut sli);
}

fn interpret2(ins: &Instruction, x: &mut HashMap<usize, Vec<char>>) {
    let from_crate = x.get_mut(&ins.from_crate).unwrap();
    let n = from_crate.len() - ins.move_n;
    let mut sli: Vec<char> = from_crate.splice(n..from_crate.len(), []).collect();
    x.get_mut(&ins.to_crate).unwrap().append(&mut sli);
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let instructions: Vec<_> = instructions.lines().map(|x| instructions_parser(x).unwrap().1).collect();
    let mut crates = to_crates(crates);
    let mut crates2 = crates.clone();
    for i in instructions.iter() {
        interpret(i, &mut crates);
    }
    for i in instructions.iter() {
        interpret2(i, &mut crates2);
    }
    let mut day5a = crates.iter().collect::<Vec<_>>();
    day5a.sort_by(|a, b| a.0.cmp(b.0));
    let mut day5b = crates2.iter().collect::<Vec<_>>();
    day5b.sort_by(|a, b| a.0.cmp(b.0));
    print!("day5a: ");
    day5a.iter().for_each(|x| { print!("{}", x.1.last().unwrap()); });
    println!();
    print!("day5b: ");
    day5b.iter().for_each(|x| { print!("{}", x.1.last().unwrap()); });
    println!();
}

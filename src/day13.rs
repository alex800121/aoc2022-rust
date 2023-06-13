use project_root::get_project_root;
use nom::{ IResult, character::complete::char, sequence::delimited, branch::alt, multi::separated_list0 };
use std::{cmp::Ordering, iter::Sum, fmt::Display};

#[derive(Debug, Clone)]
enum Mlist<T> {
    Node(T),
    List(Vec<Mlist<T>>),
}

impl<T: Display> Display for Mlist<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mlist::Node(n) => write!(f, "{}", n),
            Mlist::List(l) => {
                let mut l = l.iter();
                write!(f, "[")?;
                if let Some(a) = l.next() {
                    write!(f, "{}", a)?;
                    for a in l {
                        write!(f, ", {}", a)?;
                    }
                }
                write!(f, "]")
            },
        }
    }
}

impl<T: PartialEq> PartialEq for Mlist<T> {
    fn eq(&self, other: &Self) -> bool {
        use Mlist::*;
        match (self, other) {
            (Node(a), Node(b)) => a.eq(b),
            (List(a), List(b)) => {
                a.eq(b)
            },
            (a, List(b)) => {
                let mut b = b.iter();
                match (b.next(), b.next()) {
                    (Some(b), None) => b.eq(a),
                    (_, _) => false,
                }
            },
            (List(a), b) => {
                let mut a = a.iter();
                match (a.next(), a.next()) {
                    (Some(a), None) => a.eq(b),
                    (_, _) => false,
                }
            },
        }
    }
}

impl<T: PartialEq> Eq for Mlist<T> {}

impl<T: PartialEq + Eq + PartialOrd + Ord> PartialOrd for Mlist<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //     use Mlist::*;
    //     use Ordering::*;
    //     match (self, other) {
    //         (Node(a), Node(b)) => a.partial_cmp(b),
    //         (List(a), List(b)) => {
    //             let mut a = a.iter();
    //             let mut b = b.iter();
    //             loop {
    //                 let (x, y) = (a.next(), b.next());
    //                 match (x, y) {
    //                     (None, None) => { return Some(Equal); },
    //                     (None, Some(_)) => { return Some(Less); },
    //                     (Some(_), None) => { return Some(Greater); },
    //                     (Some(x), Some(y)) => match x.partial_cmp(y) {
    //                         Some(Equal) => { continue; },
    //                         x => { return x; },
    //                     }
    //                 }
    //             }
    //         },
    //         (a, List(b)) => {
    //             let mut b = b.iter();
    //             match (b.next(), b.next()) {
    //                 (Some(x), None) => a.partial_cmp(x),
    //                 (None, _) => Some(Greater),
    //                 (Some(x), Some(_)) => match a.partial_cmp(x) {
    //                     Some(Equal) => Some(Less),
    //                     z => z,
    //                 }
    //             }
    //         },
    //         (List(a), b) => {
    //             let mut a = a.iter();
    //             match (a.next(), a.next()) {
    //                 (Some(x), None) => x.partial_cmp(b),
    //                 (None, _) => Some(Greater),
    //                 (Some(x), Some(_)) => match x.partial_cmp(b) {
    //                     Some(Equal) => Some(Less),
    //                     z => z,
    //                 }
    //             }
    //         },
    //     }
    //     
    // }
}

impl<T: PartialEq + Eq + PartialOrd + Ord> Ord for Mlist<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Mlist::*;
        use Ordering::*;
        match (self, other) {
            (Node(a), Node(b)) => a.cmp(b),
            (List(a), List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    let (x, y) = (a.next(), b.next());
                    match (x, y) {
                        (None, None) => { return Equal; },
                        (None, Some(_)) => { return Less; },
                        (Some(_), None) => { return Greater; },
                        (Some(x), Some(y)) => match x.cmp(y) {
                            Equal => { continue; },
                            x => { return x; },
                        }
                    }
                }
            },
            (a, List(b)) => {
                let mut b = b.iter();
                match (b.next(), b.next()) {
                    (Some(x), None) => a.cmp(x),
                    (None, _) => Greater,
                    (Some(x), Some(_)) => match a.cmp(x) {
                        Equal => Less,
                        z => z,
                    }
                }
            },
            (List(a), b) => {
                let mut a = a.iter();
                match (a.next(), a.next()) {
                    (Some(x), None) => x.cmp(b),
                    (None, _) => Less,
                    (Some(x), Some(_)) => match x.cmp(b) {
                        Equal => Greater,
                        z => z,
                    }
                }
            },
        }
        
    }
}

impl<T: Sum + Copy> Mlist<T> {
    fn sum(&self) -> T {
        use Mlist::*;
        let mut v = Vec::new();
        match self {
            Node(t) => { v.push(*t); },
            List(l) => {
                for i in l { v.push(i.sum()); }
            },
        }
        v.into_iter().sum()
    }
}

fn parse_mlist(input: &str) -> IResult<&str, Mlist<usize>> {
    let (input, n) = delimited(
        char('['),
        separated_list0(char(','), alt((
            parse_num,
            parse_mlist
        ))),
        char(']')
    )(input)?;
    Ok((input, Mlist::List(n)))
}

fn parse_num(input: &str) -> IResult<&str, Mlist<usize>> {
    let (input, n) = nom::bytes::complete::take_till1(|x: char| !x.is_ascii_digit())(input)?;
    Ok((input, Mlist::Node(n.parse().unwrap())))
}

pub fn run(input: usize) {
    use Mlist::*;
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let input: Vec<_> = input.split("\n\n").map(|x| {
        x.split('\n').map(|y| parse_mlist(y).unwrap().1).collect::<Vec<_>>()
    }).collect();
    let mut n = 0;
    for (i, e) in input.iter().enumerate() {
        if e.iter().fold((true, &List(vec![])), |a, b| (a.0 && a.1 <= b, b)).0 {
            n += i + 1;
        }
    }
    println!("day13a: {}", &n);
    let mut input2 = input.concat();
    input2.push(List(vec![List(vec![Node(2)])]));
    input2.push(List(vec![List(vec![Node(6)])]));
    input2.sort();
    n = 1;
    for (i, e) in input2.iter().enumerate() {
        if e == &List(vec![List(vec![Node(6)])]) || e == &List(vec![List(vec![Node(2)])]) {
            n *= i + 1;
        }
    }
    println!("day13b: {}", &n);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn lose_to(&self) -> Self {
        use Rps::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
    fn win_to(&self) -> Self {
        use Rps::*;
        match self {
            Scissors => Paper,
            Rock => Scissors,
            Paper => Rock,
        }
    }
}

fn get_op(s: &str) -> Rps {
    use Rps::*;
    match s {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("wrong input"),
    }
}

fn strategy1 (&(op, my): &(&str, &str)) -> (Rps, Rps) {
    use Rps::*;
    let op = get_op(op);
    let my = match my {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("wrong input"),
    };
    (op, my)
}

fn strategy2 (&(op, my): &(&str, &str)) -> (Rps, Rps) {
    let op = get_op(op);
    let my = match my {
        "X" => op.win_to(),
        "Y" => op.clone(),
        "Z" => op.lose_to(),
        _ => panic!("wrong input"),
    };
    (op, my)
}

fn calc_score ((op, my): (Rps, Rps)) -> usize {
    use Rps::*;
    let win_score = if op == my.win_to() {
        6
    } else if op == my.lose_to() {
        0
    } else {
        3
    };
    let my_score = match my {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    win_score + my_score
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("../input/input{:02}.txt", input)).unwrap();
    let x: Vec<(&str, &str)> = input.lines().map(|y| y.split_once(' ').unwrap()).collect();
    let day1: Vec<usize> = x.iter().map(|x| calc_score(strategy1(x))).collect();
    let day2: Vec<usize> = x.iter().map(|x| calc_score(strategy2(x))).collect();
    println!("day2a: {}", day1.iter().sum::<usize>());
    println!("day2b: {}", day2.iter().sum::<usize>());
}

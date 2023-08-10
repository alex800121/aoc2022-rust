use std::char::from_digit;

use project_root::get_project_root;

fn from_snafu(input: &str) -> usize {
    let mut output = 0;
    for c in input.chars() {
        output *= 5;
        match c {
            '=' => {
                output -= 2;
            }
            '-' => {
                output -= 1;
            }
            c => {
                let i = c.to_digit(10).unwrap() as usize;
                output += i;
            }
        }
    }
    output
}

fn to_snafu(input: usize) -> String {
    let mut output = String::new();
    let mut rem = input % 5;
    let mut div = input / 5;
    while div + rem != 0 {
        match rem {
            3 => {
                output.insert(0, '=');
                rem = (div + 1) % 5;
                div = (div + 1) / 5;
            }
            4 => {
                output.insert(0, '-');
                rem = (div + 1) % 5;
                div = (div + 1) / 5;
            }
            i => {
                output.insert(0, from_digit(i as u32, 10).unwrap());
                rem = div % 5;
                div /= 5;
            }
        }
    }
    output
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        input
    ))
    .unwrap();
    let sum: usize = input.lines().map(from_snafu).sum();
    println!("day25a: {}", to_snafu(sum));
}

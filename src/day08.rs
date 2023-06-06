use aoc2022::Transpose;

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("../input/input{:02}.txt", input)).unwrap();
    let mut input: Vec<_> = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut transposed = input.clone().transpose();
    println!("{:?}", input);
    println!("{:?}", transposed);
}

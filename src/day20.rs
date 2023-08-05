use project_root::get_project_root;


pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input))
        .unwrap().trim().lines()
        .map(|x| x.parse::<isize>().unwrap()).enumerate().collect::<Vec<_>>();
    let len = input.len();
    dbg!(input);
}

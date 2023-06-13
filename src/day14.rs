use project_root::get_project_root;

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.split(" -> ").map(|y|{
                let (a, b) = y.split_once(',').unwrap();
                (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
            }).collect()
        })
        .collect();
    // dbg!(input);
}

use project_root::get_project_root;

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let day1 = input.split("\n\n").map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum::<u32>());
    let max_three = &mut [0; 4];
    for i in day1.into_iter() {
        max_three[0] = i;
        'inner: for j in 0..3 {
            if max_three[j] <= max_three[j + 1] {
                break 'inner;
            } else {
                max_three.swap(j, j + 1);
            }
        }
    }
    println!("day1a: {}", max_three.iter().max().unwrap());
    println!("day1b: {}", max_three[1..].iter().sum::<u32>());
}

use project_root::get_project_root;

fn step(input: &mut Vec<(usize, isize)>, n: usize, len_1: usize) {
    let removed_n = input.iter().enumerate().find(|&(_, &(i, _))| i == n).unwrap();
    let move_to = (removed_n.0 as isize + removed_n.1.1).rem_euclid(len_1 as isize) as usize;
    let removed = input.remove(removed_n.0);
    input.insert(move_to, removed);
}

pub fn run(input: usize) {
    // let mut input = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input))
    let mut input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input))
        .unwrap().trim().lines()
        .map(|x| x.parse::<isize>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    let len = input.len();
    let mut input2 = input.clone();
    input2.iter_mut().for_each(|x| x.1 *= 811589153);
    for i in 0..len {
        step(&mut input, i, len - 1);
    }
    let zero = input.iter().enumerate().find(|x| x.1.1 == 0).unwrap().0;
    let ans_a = [1000, 2000, 3000].into_iter().fold(0, |acc, x| { acc + input.get((zero + x).rem_euclid(len)).unwrap().1 });
    println!("day20a: {}", ans_a);
    for _ in 0..10 {
        for i in 0..len {
            step(&mut input2, i, len - 1);
        }
    }
    let zero = input2.iter().enumerate().find(|x| x.1.1 == 0).unwrap().0;
    let ans_b = [1000, 2000, 3000].into_iter().fold(0, |acc, x| { acc + input2.get((zero + x).rem_euclid(len)).unwrap().1 });
    println!("day20b: {}", ans_b);
}

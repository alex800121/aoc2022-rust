use project_root::get_project_root;

fn half(list: &str) -> (&str, &str) {
    let len = list.len();
    let l = len / 2;
    (&list[0..l], &list[l..len])
}

fn first_recur((fst, snd): (&str, &str)) -> u8 {
    let i = fst.find(|x| snd.contains(x)).unwrap();
    fst.as_bytes()[i]
}

fn convert(x: u8) -> usize {
    if (65..=90).contains(&x) {
        x as usize - 38
    } else {
        x as usize - 96
    }
}

fn first_recur_list(list: &[&str]) -> u8 {
    let i = &list[0];
    let j = &list[1..];
    let k = i.find(|x| j.iter().all(|y| y.contains(x))).unwrap();
    i.as_bytes()[k]
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let day3a: usize = input.lines().map(|x| convert(first_recur(half(x)))).sum();
    let day3b: usize = input.lines().collect::<Vec<_>>().chunks(3).map(|x| convert(first_recur_list(x))).sum();
    println!("day3a: {:?}", day3a);
    println!("day3b: {:?}", day3b);
}

use project_root::get_project_root;

fn n_distinct<T: PartialEq>(n: usize, input: &[T]) -> Option<usize> {
    let mut closest = Vec::new();
    for i in 0..input.len() {
        let mut j = 1;
        while j < n && i + j < input.len() && input.get(i) != input.get(i+j) {
            j += 1;
        }
        closest.push(j);
    }
    let mut i = n;
    while i < closest.len() && closest[i-n..i].iter().enumerate().any(|b| *b.1 <= n - 1 - b.0) { i += 1; };
    match i {
        i if i >= closest.len() => None,
        i => Some(i),
    }
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let input = input.chars().collect::<Vec<char>>();
    println!("day6a: {}", n_distinct(4, &input).map(|x| format!("{}", x)).unwrap_or("not found".to_owned()));
    println!("day6a: {}", n_distinct(14, &input).map(|x| format!("{}", x)).unwrap_or("not found".to_owned()));
}

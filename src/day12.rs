use project_root::get_project_root;
use std::collections::HashSet;

type Ix = (usize, usize);
fn bfs(map: &Vec<Vec<char>>, starts: &mut HashSet<Ix>, ends: &HashSet<Ix>, condition: impl Fn(&char, &char) -> bool) -> usize {
    let mut visited: HashSet<Ix> = HashSet::new();
    todo!()
}

fn condition1(start: &char, next: &char) -> bool {
    todo!()
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let mut height_map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut starts = HashSet::new();
    let mut ends = HashSet::new();
    for (i, row) in height_map.iter_mut().enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            match *e {
                'S' => { starts.insert((i, j)); },
                'E' => { ends.insert((i, j)); },
                _ => {},
            }
        }
    }
}

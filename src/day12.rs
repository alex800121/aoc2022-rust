use project_root::get_project_root;
use std::collections::{ HashSet, HashMap };

type Ix = (isize, isize);

fn bfs<I: std::fmt::Debug + Eq + std::hash::Hash>(starts: &mut HashSet<I>, ends: impl Fn(&I) -> bool, nexts: impl Fn(&I) -> HashSet<I>) -> usize {
    let mut visited: HashSet<I> = HashSet::new();
    let mut length = 0;
    loop {
        let mut next_starts = HashSet::new();
        for i in starts.drain() {
            if ends(&i) {
                return length; 
            } else {
                next_starts.extend(nexts(&i).drain().filter(|x| {
                    !visited.contains(x)
                }));
                visited.insert(i);
            }
        }
        starts.extend(next_starts.drain());
        length += 1;
    }
}

fn next_visits(height_map: &HashMap<Ix, char>, start: &Ix) -> HashSet<Ix> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|(x, y)| {
            let next = (x + start.0, y + start.1);
            let start_c = height_map.get(start)?;
            let next_c = height_map.get(&next)?;
            if start_c.to_digit(36).unwrap() + 1 >= next_c.to_digit(36).unwrap() {
                Some(next)
            } else {
                None
            }
        })
        .collect()
}

fn next_visits2(height_map: &HashMap<Ix, char>, start: &Ix) -> HashSet<Ix> {
    // let mut adjacent: HashSet<Ix> = 
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|(x, y)| {
            let next = (x + start.0, y + start.1);
            let start_c = height_map.get(start)?;
            let next_c = height_map.get(&next)?;
            // if dbg!(start_c.to_digit(36).unwrap() + 1) >= dbg!(next_c.to_digit(36).unwrap()) {
            if start_c.to_digit(36).unwrap() <= next_c.to_digit(36).unwrap() + 1 {
                Some(next)
            } else {
                None
            }
        })
        .collect()
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let mut height_map: HashMap<Ix, char> = HashMap::new();
    let mut starts = HashSet::new();
    let mut ends = HashSet::new();
    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            match char {
                'S' => {
                    starts.insert((j as isize, i as isize));
                    height_map.insert((j as isize, i as isize), 'a');
                },
                'E' => {
                    ends.insert((j as isize, i as isize));
                    height_map.insert((j as isize, i as isize), 'z');
                },
                c => {
                    height_map.insert((j as isize, i as isize), c);
                }
            }
        }
    }
    let path_length = bfs(&mut starts, |x| ends.contains(x), |x| next_visits(&height_map, x));
    println!("day12a: {}", path_length);
    let path_length = bfs(&mut ends, |x| height_map.get(x) == Some(&'a'), |x| next_visits2(&height_map, x));
    println!("day12b: {}", path_length);
}

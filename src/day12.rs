use project_root::get_project_root;
use std::collections::{ HashSet, HashMap };
use aoc2022::bfs;

type Ix = (isize, isize);

fn next_visits(height_map: &HashMap<Ix, char>, start: &(Ix, usize), results: &mut HashMap<Ix, usize>) -> HashMap<Ix, usize> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|(x, y)| {
            let next = (x + start.0.0, y + start.0.1);
            let start_c = height_map.get(&start.0)?;
            let next_c = height_map.get(&next)?;
            // dbg!(&results.len());
            if start_c.to_digit(36).unwrap() + 1 >= next_c.to_digit(36).unwrap() {
                if let Some(n) = results.get(&next) {
                    if start.1 + 1 < *n {
                        results.insert(next, start.1 + 1);
                        Some((next, start.1 + 1))
                    } else {
                        None
                    }
                } else {
                    results.insert(next, start.1 + 1);
                    Some((next, start.1 + 1))
                }
            } else {
                None
            }
        })
        .collect()
}

fn next_visits2(height_map: &HashMap<Ix, char>, start: &(Ix, usize), results: &mut HashMap<Ix, usize>) -> HashMap<Ix, usize> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|(x, y)| {
            let next = (x + start.0.0, y + start.0.1);
            let start_c = height_map.get(&start.0)?;
            let next_c = height_map.get(&next)?;
            // dbg!(&results.len());
            if start_c.to_digit(36).unwrap() <= next_c.to_digit(36).unwrap() + 1 {
                if let Some(n) = results.get(&next) {
                    if start.1 + 1 < *n {
                        results.insert(next, start.1 + 1);
                        Some((next, start.1 + 1))
                    } else {
                        None
                    }
                } else {
                    results.insert(next, start.1 + 1);
                    Some((next, start.1 + 1))
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let mut height_map: HashMap<Ix, char> = HashMap::new();
    let mut starts = HashMap::new();
    let mut ends = HashSet::new();
    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            match char {
                'S' => {
                    starts.insert((j as isize, i as isize), 0);
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
    let path = bfs(starts.clone(), |x| x.iter().any(|y| ends.contains(y.0)), |x, y| next_visits(&height_map, x, y));
    println!("day12a: {}", path.values().max().unwrap());
    let path = bfs(HashMap::from_iter(ends.into_iter().map(|x| (x, 0))), |x| x.iter().any(|y| height_map.get(y.0) == Some(&'a')), |x, y| next_visits2(&height_map, x, y));
    println!("day12b: {}", path.values().max().unwrap());
}

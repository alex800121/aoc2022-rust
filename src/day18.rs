use std::collections::HashSet;

use project_root::get_project_root;

type Obsidian = [isize; 3];
const ADJACENTS: [Obsidian; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn manhattan<const N: usize>(a: [isize; N], b: [isize; N]) -> isize {
    (0..N).fold(0, |acc, x| acc + (a[x] - b[x]).abs())
}

fn count_sides(obsidians: &[Obsidian]) -> usize {
    let mut adjacent_sides: usize = 0;
    for i in 0..obsidians.len() {
        for j in i..obsidians.len() {
            if manhattan(obsidians[i], obsidians[j]) == 1 {
                adjacent_sides += 1;
            }
        }
    }
    obsidians.len() * 6 - adjacent_sides * 2
}

fn count_surfaces(obsidians: &HashSet<Obsidian>) -> usize {
    let mut travelled: HashSet<Obsidian> = HashSet::new();
    let first = *obsidians.iter().next().unwrap();
    let (min, max) = obsidians.iter().fold((first, first), |(min, max), &x| {
        (
            [
                min[0].min(x[0]),
                min[1].min(x[1]),
                min[2].min(x[2])
            ],
            [
                max[0].max(x[0]),
                max[1].max(x[1]),
                max[2].max(x[2])
            ]
        )
    });
    let (min, max) = (min.map(|x| x - 1), max.map(|x| x + 1));
    let mut sides = (0..3).map(|i| max[i] - min[i] + 1);
    let sides = [sides.next().unwrap(), sides.next().unwrap(), sides.next().unwrap()]; 
    let outer_surface = [
        sides[0] * sides[1],
        sides[2] * sides[1],
        sides[0] * sides[2]
    ].iter().sum::<isize>() * 2;
    let mut start: HashSet<Obsidian> = HashSet::from([min, max]);
    while !start.is_empty() {
        let mut next: HashSet<Obsidian> = HashSet::new();
        for i in start.drain() {
            travelled.insert(i);
            next.extend(ADJACENTS
                .map(|x| [x[0] + i[0], x[1] + i[1], x[2] + i[2]])
                .iter()
                .filter(|&a| !travelled.contains(a) && !obsidians.contains(a) && (0..3usize).all(|x| a[x] >= min[x] && a[x] <= max[x]))
            );
        }
        start = next;
    }
    count_sides(&travelled.into_iter().collect::<Vec<_>>()) - outer_surface as usize
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let obsidians: Vec<Obsidian> = input
        .lines()
        .map(|x| {
            let mut v = x
                .split(',')
                .map(|y| y
                    .parse::<isize>()
                    .unwrap()
                );
            [v.next().unwrap(), v.next().unwrap(), v.next().unwrap()]
        })
        .collect();
    println!("day18a: {}", count_sides(&obsidians));
    let obsidians: HashSet<_> = HashSet::from_iter(obsidians);
    println!("day18b: {}", count_surfaces(&obsidians));
}

use aoc2022::{build_map, print_map};
use project_root::get_project_root;
use std::{
    array::from_fn,
    collections::{BTreeMap, BTreeSet},
};

type Index = (isize, isize);
const ADJACENT: [Index; 5] = [(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1)];

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        input
    ))
    .unwrap();
    let input = input.trim().lines().map(|x| x.chars());
    let height = input.clone().count() as isize;
    let width = input.clone().next().unwrap().count() as isize;
    let wind_height = height - 2;
    let wind_width = width - 2;
    let map = build_map(input, |x, y| {
        Some(((x.0 as isize, x.1 as isize), y)).into_iter()
    });
    let mut start = BTreeSet::from([(1, 0)]);
    let mut start2 = BTreeSet::from([((1, 0), 0)]);
    let end = (width - 2, height - 1);
    let end2 = [(end, 2), ((1, 0), 1), (end, 0)];
    let is_open = |i @ (x, y): Index, n: isize| {
        let north_wind = (x, (y - 1 - n).rem_euclid(wind_height) + 1);
        let south_wind = (x, (y - 1 + n).rem_euclid(wind_height) + 1);
        let east_wind = ((x - 1 + n).rem_euclid(wind_width) + 1, y);
        let west_wind = ((x - 1 - n).rem_euclid(wind_width) + 1, y);
        i == (1, 0)
            || i == (width - 2, height - 1)
            || (map.get(&i).is_some_and(|c| *c != '#')
                && map.get(&north_wind).is_some_and(|c| *c != 'v')
                && map.get(&south_wind).is_some_and(|c| *c != '^')
                && map.get(&east_wind).is_some_and(|c| *c != '<')
                && map.get(&west_wind).is_some_and(|c| *c != '>'))
    };
    let mut i = 0;
    while !start.contains(&end) {
        i += 1;
        start = BTreeSet::from_iter(start.into_iter().flat_map(|x: Index| {
            ADJACENT
                .map(|y| (y.0 + x.0, y.1 + x.1))
                .into_iter()
                .filter(|x| is_open(*x, i))
        }));
    }
    println!("day24a: {}", i);
    let mut i = 0;
    while !start2.contains(&(end, 3)) {
        i += 1;
        start2 = BTreeSet::from_iter(start2.into_iter().flat_map(|(x, z)| {
            ADJACENT
                .map(|y| ((y.0 + x.0, y.1 + x.1), z))
                .into_iter()
                .filter(|x| is_open(x.0, i))
        }));
        for x in end2.iter() {
            if start2.remove(x) {
                start2.insert((x.0, x.1 + 1));
            }
        }
    }
    println!("day24b: {}", i);
}

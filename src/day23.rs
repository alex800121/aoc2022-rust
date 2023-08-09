use aoc2022::{
    build_map,
    Direction::{self, *},
    Enum,
};
use project_root::get_project_root;
use std::collections::BTreeMap;

type Index = (isize, isize);

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input
    )).unwrap();
    let input = input.trim().lines().map(|x| x.chars());
    let elves = build_map(input, |k, x| {
        let k = (k.0 as isize, k.1 as isize);
        match x {
           '#' => Some((k, k)).into_iter(),
            _ => None.into_iter()
        }
    });
}

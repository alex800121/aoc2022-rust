use aoc2022::{
    build_map,
    Direction::{self, *},
    Enum,
};
use project_root::get_project_root;
use std::{collections::{BTreeMap, BTreeSet}, array::from_fn};

type Index = (isize, isize);
type Elves = BTreeMap<Index, Index>;
type Forbidden = BTreeSet<Index>;

const ADJACENT: [Index; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1)
];

const PROPOSAL: [[Index; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 0)],
    [(1, -1), (1, 0), (1, 0)],
];

fn propose_move(elves: &mut Elves, step: usize) {
    let mut forbidden: Forbidden = BTreeSet::new();
    let mut next_moves: Elves = BTreeMap::new();
    let proposal: [[Index; 3]; 4] = from_fn(|n| PROPOSAL[n % 4]);
    'outer: for (ori, _) in elves.iter() {
        if ADJACENT.iter().all(|x| !elves.contains_key(&(x.0 + ori.0, x.1 + ori.1))) {
            next_moves.insert(*ori, *ori);
        } else {
            'inner: for direction in proposal {
                let direction = direction.map(|x| (x.0 + ori.0, x.1 + ori.1));
                let proposed = direction[1];
                if direction.iter().all(|x| !elves.contains_key(x)) {
                    if let Some(duplicate_ori) = next_moves.get(&proposed) {
                    }
                }
            }
        }
    }
}

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

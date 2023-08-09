use aoc2022::{
    build_map,
    print_map,
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
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];

fn propose_move(elves: &Elves, step: usize) -> Elves {
    let mut forbidden: Forbidden = BTreeSet::new();
    let mut next_moves: Elves = BTreeMap::new();
    let proposal: [[Index; 3]; 4] = from_fn(|n| PROPOSAL[(n + step) % 4]);
    'outer: for (ori, _) in elves.iter() {
        if ADJACENT.iter().all(|x| !elves.contains_key(&(x.0 + ori.0, x.1 + ori.1))) {
            next_moves.insert(*ori, *ori);
        } else {
            for direction in proposal {
                let direction = direction.map(|x| (x.0 + ori.0, x.1 + ori.1));
                let proposed = direction[1];
                if direction.iter().all(|x| !elves.contains_key(x)) {
                    if let Some(duplicate_ori) = next_moves.get(&proposed) {
                        forbidden.insert(proposed);
                        next_moves.insert(*duplicate_ori, *duplicate_ori);
                        next_moves.insert(*ori, *ori);
                        next_moves.remove(&proposed);
                    } else if forbidden.contains(&proposed) {
                        next_moves.insert(*ori, *ori);
                    } else {
                        next_moves.insert(proposed, *ori);
                    }
                    continue 'outer;
                }
            }
            next_moves.insert(*ori, *ori);
        }
    }
    next_moves
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input
        "{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input
    )).unwrap();
    let input = input.trim().lines().map(|x| x.chars());
    let mut elves = build_map(input, |k, x| {
        let k = (k.0 as isize, k.1 as isize);
        match x {
           '#' => Some((k, k)).into_iter(),
            _ => None.into_iter()
        }
    });
    for i in 0..10 {
        elves = propose_move(&elves, i);
    }
    let result = print_map(&elves, |x| {
        match x {
            None => '.',
            Some(_) => '#',
        }
    });
    println!("day23a: {}", result.chars().filter(|x| *x == '.').count());
    let mut i = 10;
    loop {
        let elves2 = propose_move(&elves, i);
        if elves2.keys().collect::<Vec<_>>() == elves.keys().collect::<Vec<_>>() {
            break;
        }
        elves = elves2;
        i += 1;
    }
    println!("day23b: {}", i + 1);
}

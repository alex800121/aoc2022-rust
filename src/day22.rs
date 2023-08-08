use aoc2022::{
    build_map,
    Direction::{self, *},
    Enum, ZipWith,
};
use nom::AsChar;
use num::Integer;
use project_root::get_project_root;
use std::{collections::BTreeMap, array::from_fn};

type Index = (isize, isize);
type IndexPlus = (Index, Index);
type CubeMap = BTreeMap<Index, bool>;
type CubeMapPlus = BTreeMap<IndexPlus, bool>;
type RawCubeSide = BTreeMap<Index, [Option<(Index, Direction)>; 4]>;
type CubeSide = BTreeMap<Index, [(Index, Direction); 4]>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Instruction {
    Right,
    Left,
    Forward(isize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Bot<T> {
    position: T,
    direction: Direction,
}

impl Bot<Index> {
    fn next(&mut self, map: &CubeMap, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Right => {
                self.direction = self.direction.succ();
            }
            Left => {
                self.direction = self.direction.pred();
            }
            Forward(i) => {
                let move_to = |(x, y): Index| {
                    match self.direction {
                        North => ((x, y - 1), *map.keys().filter(|k| k.0 == x).max_by(|a, b| a.1.cmp(&b.1)).unwrap()),
                        East => ((x + 1, y), *map.keys().filter(|k| k.1 == y).min_by(|a, b| a.0.cmp(&b.0)).unwrap()),
                        South => ((x, y + 1), *map.keys().filter(|k| k.0 == x).min_by(|a, b| a.1.cmp(&b.1)).unwrap()),
                        West => ((x - 1, y), *map.keys().filter(|k| k.1 == y).max_by(|a, b| a.0.cmp(&b.0)).unwrap()),
                    }
                };
                for _ in 0..*i {
                    let (next1, next2) = move_to(self.position);
                    match (map.get(&next1), map.get(&next2)) {
                        (Some(true), _) => { self.position = next1; },
                        (None, Some(true)) => { self.position = next2; },
                        _ => { break; },
                    }
                }
            }
        }
    }
}

impl Bot<IndexPlus> {
    fn next(&mut self, map: &CubeMapPlus, instruction: &Instruction) {}
}

fn fold_cube_side(mut raw_cube_side: RawCubeSide) -> Option<RawCubeSide> {
    let sides: Vec<_> = raw_cube_side.keys().copied().collect();
    let adjacent = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for side in sides.iter() {
        for (direction, index) in adjacent.iter().enumerate() {
            let to_direction = (side.0 + index.0, side.1 + index.1);
            if sides.contains(&to_direction) {
                let t = raw_cube_side.get_mut(side)?;
                t[direction] = Some((to_direction, Enum::to_enum(direction as isize)));
            }
        }
    }
    Some(raw_cube_side)
}

pub fn run(input: usize) {
    use Instruction::*;
    let (map1, map2, instructions): (CubeMap, CubeMapPlus, Vec<Instruction>) =
        std::fs::read_to_string(format!(
            "{}/input/test{:02}.txt",
            // "{}/input/input{:02}.txt",
            get_project_root().unwrap().to_str().unwrap(),
            input
        ))
        .ok()
        .and_then(|x| {
            let (map, instructions) = x.split_once("\n\n")?;
            let map = map.lines().map(String::from).collect::<Vec<_>>();
            let instructions = instructions
                .trim()
                .split_inclusive(|x: char| !x.is_dec_digit());

            let mut i: Vec<Instruction> = Vec::new();
            for j in instructions {
                if j.ends_with('R') {
                    i.push(Forward(
                        j.strip_suffix('R').and_then(|x| x.parse().ok()).unwrap(),
                    ));
                    i.push(Right);
                } else if j.ends_with('L') {
                    i.push(Forward(
                        j.strip_suffix('L').and_then(|x| x.parse().ok()).unwrap(),
                    ));
                    i.push(Left);
                } else {
                    i.push(Forward(j.parse().ok().unwrap()));
                }
            }
            let height = map.len();
            let width = map.iter().map(|x| x.len()).max().unwrap();
            let side = height.gcd(&width);
            let map1 = build_map(
                map.iter().map(|x| x.chars()),
                |(x, y)| (x as isize, y as isize),
                |c| match c {
                    '.' => Some(true),
                    '#' => Some(false),
                    _ => None,
                },
            );
            let map2 = build_map(
                map.iter().map(|x| x.chars()),
                |(x, y)| {
                    (
                        ((x / side) as isize, (y / side) as isize),
                        ((x % side) as isize, (y % side) as isize),
                    )
                },
                |c| match c {
                    '.' => Some(true),
                    '#' => Some(false),
                    _ => None,
                },
            );
            let raw_cube_side: RawCubeSide = BTreeMap::from_iter(map2.keys().map(|x| (x.0, [None; 4])));
            Some((map1, map2, i))
        })
        .unwrap();
    let mut init_bot1: Bot<Index> = Bot {
        position: *map1
            .keys()
            .min_by(|x, y| x.1.cmp(&y.1).then(x.0.cmp(&y.0)))
            .unwrap(),
        direction: East,
    };
    let mut init_bot2 = Bot {
        position: map2
            .keys()
            .min_by(|x, y| {
                x.0 .1
                    .cmp(&y.0 .1)
                    .then(x.0 .0.cmp(&y.0 .0))
                    .then(x.1 .1.cmp(&y.1 .1))
                    .then(x.1 .0.cmp(&y.1 .0))
            })
            .unwrap(),
        direction: East,
    };
    for i in instructions.iter() {
        init_bot1.next(&map1, i);
    }
    println!("day22a: {}", (init_bot1.position.1 + 1) * 1000 + (init_bot1.position.0 + 1) * 4 + (init_bot1.direction.to_int() - 1).rem_euclid(4));
}

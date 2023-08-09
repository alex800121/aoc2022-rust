use aoc2022::{
    build_map,
    Direction::{self, *},
    Enum,
};
use nom::AsChar;
use num::Integer;
use project_root::get_project_root;
use std::collections::BTreeMap;

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
    fn next(&mut self, map: &CubeMapPlus, cube_side: &CubeSide, len: usize, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Right => {
                self.direction = self.direction.succ();
            }
            Left => {
                self.direction = self.direction.pred();
            }
            Forward(i) => {
                let move_to = |(s, (x, y)): IndexPlus, d: Direction| {
                    match d {
                        North => ((s, (x, y - 1)), cross_side((s, (x, y)), d, cube_side, len)),
                        East => ((s, (x + 1, y)), cross_side((s, (x, y)), d, cube_side, len)),
                        South => ((s, (x, y + 1)), cross_side((s, (x, y)), d, cube_side, len)),
                        West => ((s, (x - 1, y)), cross_side((s, (x, y)), d, cube_side, len)),
                    }
                };
                for _ in 0..*i {
                    let (next1, (next2, to)) = move_to(self.position, self.direction);
                    match (map.get(&next1), map.get(&next2)) {
                        (Some(true), _) => { 
                            self.position = next1;
                        },
                        (None, Some(true)) => { 
                            self.position = next2;
                            self.direction = to;
                        },
                        _ => { break; },
                    }
                }
            }
        }

    }
}

fn fold_cube_side(mut raw_cube_side: RawCubeSide) -> CubeSide {
    let sides: Vec<_> = raw_cube_side.keys().copied().collect();
    let adjacent = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for side in sides.iter() {
        for (direction, index) in adjacent.iter().enumerate() {
            let to_direction = (side.0 + index.0, side.1 + index.1);
            if sides.contains(&to_direction) {
                let t = raw_cube_side.get_mut(side).unwrap();
                t[direction] = Some((to_direction, Enum::to_enum(direction as isize)));
            }
        }
    }
    while raw_cube_side.values().any(|x| x.iter().any(|y| y.is_none())) {
        for side in sides.iter() {
            'inner: for direction1 in 0..4usize {
                let direction2 = (direction1 + 1).rem_euclid(4);
                let center = raw_cube_side.get(side).unwrap();
                match (center[direction1], center[direction2]) {
                    (Some((i1, d1)), Some((i2, d2))) => {
                        let s1 = raw_cube_side.get_mut(&i1).unwrap();
                        s1[(d1.to_int() + 1).rem_euclid(4) as usize] = Some((i2, Enum::to_enum((d2.to_int() + 1).rem_euclid(4))));
                        let s2 = raw_cube_side.get_mut(&i2).unwrap();
                        s2[(d2.to_int() - 1).rem_euclid(4) as usize] = Some((i1, Enum::to_enum((d1.to_int() - 1).rem_euclid(4))));
                    },
                    _ => { continue 'inner; },
                }
            }
        }
    }
    let mut cube_side: CubeSide = BTreeMap::new();
    for (k, e) in raw_cube_side.into_iter() {
        let mut v = Vec::new();
        for i in e {
            v.push(i.unwrap());
        }
        let v = v.try_into().ok().unwrap();
        cube_side.insert(k, v);
    }
    cube_side
}

fn cross_side(index: IndexPlus, from: Direction, cube_side: &CubeSide, len: usize) -> (IndexPlus, Direction) {
    let len = len as isize;
    let (next_side, to) = cube_side.get(&index.0).unwrap()[from.to_int() as usize];
    let next_index = match (from, to) {
        (East, West) | (West, East) | (North, North) | (South, South) => {
            (index.1.0, len - 1 - index.1.1)
        },
        (North, South) | (South, North) | (East, East) | (West, West) => {
            (len - 1 - index.1.0, index.1.1)
        },
        (North, East) | (East, North) | (South, West) | (West, South) => {
            (index.1.1, index.1.0)
        },
        (North, West) | (West, North) | (South, East) | (East, South) => {
            (len - 1 - index.1.1, len - 1 - index.1.0)
        },
    };
    ((next_side, next_index), to)
}

pub fn run(input: usize) {
    use Instruction::*;
    let (map1, map2, cube_side, len, instructions): (CubeMap, CubeMapPlus, CubeSide, usize, Vec<Instruction>) =
        std::fs::read_to_string(format!(
            // "{}/input/test{:02}.txt",
            "{}/input/input{:02}.txt",
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
                |(x, y), c| match c {
                    '.' => Some(((x as isize, y as isize), true)).into_iter(),
                    '#' => Some(((x as isize, y as isize), false)).into_iter(),
                    _ => None.into_iter(),
                },
            );
            let map2 = build_map(
                map.iter().map(|x| x.chars()),
                |(x, y), c| { 
                    let key = (
                        ((x / side) as isize, (y / side) as isize),
                        ((x % side) as isize, (y % side) as isize)
                    );
                    match c {
                        '.' => Some((key, true)).into_iter(),
                        '#' => Some((key, false)).into_iter(),
                        _ => None.into_iter(),
                    }
                }
            );
            let raw_cube_side: RawCubeSide = BTreeMap::from_iter(map2.keys().map(|x| (x.0, [None; 4])));
            let cube_side = fold_cube_side(raw_cube_side);
            Some((map1, map2, cube_side, side, i))
        })
        .unwrap();
    let mut init_bot1: Bot<Index> = Bot {
        position: *map1
            .keys()
            .min_by(|x, y| x.1.cmp(&y.1).then(x.0.cmp(&y.0)))
            .unwrap(),
        direction: East,
    };
    let mut init_bot2: Bot<IndexPlus> = Bot {
        position: *map2
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
    for i in instructions.iter() {
        init_bot2.next(&map2, &cube_side, len, i);
    }
    let final_position = (
        init_bot2.position.0.0 * len as isize + init_bot2.position.1.0,
        init_bot2.position.0.1 * len as isize + init_bot2.position.1.1,
    );
    println!("day22b: {}", (final_position.1 + 1) * 1000 + (final_position.0 + 1) * 4 + (init_bot2.direction.to_int() - 1).rem_euclid(4));
}

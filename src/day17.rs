use project_root::get_project_root;
use std::{default::Default, fmt::Display, collections::HashSet};

const SHAPE1: [u16; 4] = [
    0b00111100,
    0b00000000,
    0b00000000,
    0b00000000
];
const SHAPE2: [u16; 4] = [
    0b00010000,
    0b00111000,
    0b00010000,
    0b00000000
];
const SHAPE3: [u16; 4] = [
    0b00111000,
    0b00001000,
    0b00001000,
    0b00000000
];
const SHAPE4: [u16; 4] = [
    0b00100000,
    0b00100000,
    0b00100000,
    0b00100000
];
const SHAPE5: [u16; 4] = [
    0b00110000,
    0b00110000,
    0b00000000,
    0b00000000
];
const SHAPE_LIST: [[u16; 4]; 5] = [SHAPE1, SHAPE2, SHAPE3, SHAPE4, SHAPE5];
const WALL: u16  = 0b100000001;
const FLOOR: u16 = 0b111111111;
const DAYB: usize = 1000000000000;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GameState {
    board: [u16; 64],
    height: usize,
}
impl Default for GameState {
    fn default() -> Self {
        let mut board = [WALL; 64];
        *board.get_mut(63).unwrap() = FLOOR;
        GameState {
            board,
            height: 0,
        }
    }
}
impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in 0..64 {
            writeln!(f, "{:09b}", self.board[63 - line])?;
        }
        writeln!(f, "height: {}", self.height)
    }
}

impl GameState {
    fn init(&mut self) {
        for i in 0..8 {
            self.board[i] = WALL;
        }
    }

    fn drop_shape_list<S, T>(&mut self, shape_list: &mut S, wind: &mut T, cycle: usize) where
        S: Iterator<Item = [u16; 4]>,
        T: Iterator<Item = char> {
        for _ in 0..cycle {
            if let Some(current_shape) = shape_list.next() {
                self.drop_shape(current_shape, wind);
            } else {
                break;
            }
        }
    }

    fn drop_shape<T>(&mut self, mut current_shape: [u16; 4], wind: &mut T) where
        T: Iterator<Item = char> {
        self.init();
        let mut current_floor: usize = 3;
        for _ in 0..64 {
            let blow = wind.next().unwrap();
            match blow {
                '>' => {
                    current_shape.iter_mut().for_each(|x| *x >>= 1);
                    if current_shape.iter().enumerate().any(|x| {
                        (x.1 & self.board[(current_floor + x.0) % 64]) != 0
                    }) {
                        current_shape.iter_mut().for_each(|x| *x <<= 1);
                    }
                },
                _ => {
                    current_shape.iter_mut().for_each(|x| *x <<= 1);
                    if current_shape.iter().enumerate().any(|x| {
                        0 != (x.1 & self.board[(current_floor + x.0) % 64])
                    }) {
                        current_shape.iter_mut().for_each(|x| *x >>= 1);
                    }
                }
            }
            current_floor = (current_floor + 63) % 64;
            if current_shape.iter().enumerate().any(|x| {
                (x.1 & self.board[(current_floor + x.0) % 64]) != 0
            }) {
                current_floor = (current_floor + 1) % 64;
                for (i, new_line) in current_shape.into_iter().enumerate() {
                    *self.board.get_mut((current_floor + i) % 64).unwrap() |= new_line;
                }
                break;
            }
        }
        while self.board[0] != WALL {
            self.height += 1;
            self.board.rotate_left(1);
        }
    }
}

pub fn run(input: usize) {
    // let input = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let mut wind = input[..].chars().cycle();
    let mut shape_list = SHAPE_LIST.into_iter().cycle();
    let mut gamestate = GameState::default();
    gamestate.drop_shape_list(&mut shape_list, &mut wind, 2022);
    println!("day17a: {}", gamestate.height);
    let mut wind = input[..].chars().cycle();
    let mut shape_list = SHAPE_LIST.into_iter().cycle();
    let mut gamestate = GameState::default();
    let mut cache: HashSet<[u16; 64]> = HashSet::new();
    let first_repeat = loop {
        if cache.contains(&gamestate.board) {
            break (cache.len(), gamestate);
        }
        cache.insert(gamestate.board);
        gamestate.drop_shape(shape_list.next().unwrap(), &mut wind);
    };
    // println!("{}\n{}", first_repeat.0, first_repeat.1);
    let mut n = 0;
    gamestate.height = 0;
    let repeat_cycle = loop {
        n += 1;
        gamestate.drop_shape(shape_list.next().unwrap(), &mut wind);
        if gamestate.board == first_repeat.1.board {
            break (n, gamestate);
        }
    };
    let (cycle, remaining) = ((DAYB - first_repeat.0) / repeat_cycle.0, (DAYB - first_repeat.0) % repeat_cycle.0);
    gamestate.height = 0;
    gamestate.drop_shape_list(&mut shape_list, &mut wind, remaining);
    println!("day17b: {}", first_repeat.1.height + (cycle * repeat_cycle.1.height) + gamestate.height);
}

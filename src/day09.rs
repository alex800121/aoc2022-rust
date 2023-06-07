use project_root::get_project_root;
use aoc2022::{ Direction, Direction::* };
use std::collections::HashSet;

type Position = (isize, isize);
struct Rope {
    head: Position,
    tail: Vec<Position>
}

struct Instruction {
    direction: Direction,
    step: isize,
}
impl Rope {
    fn move_head(&mut self, direction: &Direction) {
        let (x, y) = match direction {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };
        self.head.0 += x; 
        self.head.1 += y;
    }
    fn follow_head(&mut self) {
        let mut head = self.head;
        self.tail.iter_mut().for_each(|x| {
            if !near(*x, head) {
                let move_x = (head.0 - x.0).signum();
                let move_y = (head.1 - x.1).signum();
                x.0 += move_x;
                x.1 += move_y;
            }
            head = *x;
        });
    }
    fn read_instruction(&mut self, instruction: &Instruction, tail_acc: &mut HashSet<Position>) {
        for _ in 0..instruction.step {
            self.move_head(&instruction.direction);
            self.follow_head();
            tail_acc.insert(*self.tail.last().unwrap());
        }
    }
}

fn near(a: Position, b: Position) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}
pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let instructions: Vec<_> = input.lines().map(|x| {
        let (d, s) = x.split_once(' ').unwrap();
        Instruction {
            direction: match d {
                "R" => East,
                "D" => South,
                "L" => West,
                _ => North,
            }, 
            step: s.parse::<isize>().unwrap(),
        }
    }).collect();
    let mut rope = Rope { head: (0, 0), tail: vec![(0, 0)]};
    let mut tail_acc = HashSet::new();
    instructions.iter().for_each(|x| rope.read_instruction(x, &mut tail_acc));
    println!("day9a: {}", tail_acc.len());
    let mut rope = Rope { head: (0, 0), tail: vec![(0, 0); 9]};
    let mut tail_acc = HashSet::new();
    instructions.iter().for_each(|x| rope.read_instruction(x, &mut tail_acc));
    println!("day9b: {}", tail_acc.len());
}

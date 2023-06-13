use project_root::get_project_root;
use std::collections::HashSet;

type Ix = (isize, isize);
type Wall = HashSet<Ix>;

fn drop_sand(wall: &mut Wall, start: &Ix, stop: impl Fn(&Ix) -> bool) -> usize {
    todo!()
}

fn build_wall(wall: &mut Wall, instruction: &[Ix]) {
    let mut instruction = instruction.iter();
    if let Some(init) = instruction.next() {
        let mut init = *init;
        wall.insert(init);
        for next in instruction {
            let interval = ((next.0 - init.0).signum(), (next.1 - init.1).signum());
            while init != *next {
                init = (init.0 + interval.0, init.1 + interval.1);
                wall.insert(init);
            }
        }
    }
}

fn draw_wall(wall: &Wall) -> String {
    let minx = wall.iter().min_by(|&a, &b| a.0.cmp(&b.0)).unwrap().0;
    let maxx = wall.iter().max_by(|&a, &b| a.0.cmp(&b.0)).unwrap().0;
    let miny = wall.iter().min_by(|&a, &b| a.1.cmp(&b.1)).unwrap().1;
    let maxy = wall.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap().1;
    let mut s = String::new();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if wall.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.split(" -> ").map(|y|{
                let (a, b) = y.split_once(',').unwrap();
                (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
            }).collect()
        })
        .collect();
    let mut wall: Wall = HashSet::new();
    input.iter().for_each(|x| { build_wall(&mut wall, x); });
    dbg!(wall);
}

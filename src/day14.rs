use project_root::get_project_root;
use std::collections::HashSet;

type Ix = (isize, isize);
type Wall = HashSet<Ix>;

fn drop_sand(wall: &mut Wall, start: &Ix) -> usize {
    let mut sand_count = 0;
    let mut next_sand = *start;
    let maxy = wall.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap().1;
    loop {
        if !wall.contains(&(next_sand.0, next_sand.1 + 1)) {
            next_sand = (next_sand.0, next_sand.1 + 1);
        } else if !wall.contains(&(next_sand.0 - 1, next_sand.1 + 1)) {
            next_sand = (next_sand.0 - 1, next_sand.1 + 1);
        } else if !wall.contains(&(next_sand.0 + 1, next_sand.1 + 1)) {
            next_sand = (next_sand.0 + 1, next_sand.1 + 1);
        } else {
            sand_count += 1;
            wall.insert(next_sand);
            if next_sand.1 >= maxy { break; }
            next_sand = *start;
            continue;
        }
        if next_sand.1 >= maxy { break; }
    }
    // println!("{}", draw_wall(wall));
    sand_count
}

fn drop_sand3(wall: &mut Wall, start: &mut HashSet<isize>, init_y: isize) -> usize {
    let mut sand_count = 0;
    let mut next_start: HashSet<isize> = HashSet::new();
    let maxy = wall.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap().1;
    for y in init_y..maxy + 2 {
        for i in start.drain() {
            sand_count += 1;
            ((i - 1)..=(i + 1)).filter(|x| !wall.contains(&(*x, y + 1))).for_each(|x| {
                next_start.insert(x);
            });
        }
        start.extend(next_start.drain());
    }
    sand_count
}

// fn drop_sand2(wall: &mut Wall, start: &Ix) -> usize {
//     let mut sand_count = 0;
//     let mut next_sand = *start;
//     let maxy = wall.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap().1;
//     loop {
//         if next_sand.1 > maxy {
//             sand_count += 1;
//             wall.insert(next_sand);
//             next_sand = *start;
//             continue;
//         } else if !wall.contains(&(next_sand.0, next_sand.1 + 1)) {
//             next_sand = (next_sand.0, next_sand.1 + 1);
//         } else if !wall.contains(&(next_sand.0 - 1, next_sand.1 + 1)) {
//             next_sand = (next_sand.0 - 1, next_sand.1 + 1);
//         } else if !wall.contains(&(next_sand.0 + 1, next_sand.1 + 1)) {
//             next_sand = (next_sand.0 + 1, next_sand.1 + 1);
//         } else {
//             sand_count += 1;
//             wall.insert(next_sand);
//             if (500, 0) == next_sand { break; }
//             next_sand = *start;
//             continue;
//         }
//         if (500, 0) == next_sand { break; }
//     }
//     // println!("{}", draw_wall(wall));
//     sand_count
// }

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

#[allow(dead_code)]
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
    // println!("{}", draw_wall(&wall));
    // dbg!(wall);
    let n = drop_sand(&mut wall.clone(), &(500, 0));
    println!("day14a: {}", n);
    let mut start = HashSet::from([500]);
    // let n = drop_sand2(&mut wall.clone(), &(500, 0), );
    // println!("day14b: {}", n);
    let n = drop_sand3(&mut wall, &mut start, 0);
    println!("day14b: {}", n);
}

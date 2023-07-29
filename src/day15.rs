use aoc2022::{reduce_sorted_range, EucVec};
use nom::{
    bytes::complete::{tag, take_while},
    IResult,
};
use project_root::get_project_root;
use std::collections::HashSet;
use std::ops::Range;

type Ix = (isize, isize);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    position: Ix,
    man: isize,
}

// #[derive(Clone, Debug, Hash, PartialEq, Eq)]
// struct Diamond {
//     y_plus_x: Range<isize>,
//     y_minus_x: Range<isize>,
// }
type Diamond = [Range<isize>; 2];

impl Sensor {
    fn to_diamond(&self) -> Diamond {
        let man = self.man;
        let a = self.position.0 + self.position.1;
        let b = self.position.1 - self.position.0;
        [(a - man)..(a + man + 1), (b - man)..(b + man + 1)]
    }
}

fn calc_center(d: Diamond) -> Ix {
    let a = (d[0].start, d[0].end);
    let b = (d[1].start, d[1].end);
    let y_plus_x = (a.0 + a.1 - 1) / 2;
    let y_minus_x = (b.0 + b.1 - 1) / 2;
    ((y_plus_x - y_minus_x) / 2, (y_plus_x + y_minus_x) / 2)
}

fn parse_input(input: &str) -> IResult<&str, (Sensor, Ix)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, s_x) = take_while(|x: char| x.is_ascii_digit() || x == '-')(input)?;
    let s_x = s_x.parse::<isize>().unwrap();
    let (input, _) = tag(", y=")(input)?;
    let (input, s_y) = take_while(|x: char| x.is_ascii_digit() || x == '-')(input)?;
    let s_y = s_y.parse::<isize>().unwrap();
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, b_x) = take_while(|x: char| x.is_ascii_digit() || x == '-')(input)?;
    let b_x = b_x.parse::<isize>().unwrap();
    let (input, _) = tag(", y=")(input)?;
    let (input, b_y) = take_while(|x: char| x.is_ascii_digit() || x == '-')(input)?;
    let b_y = b_y.parse::<isize>().unwrap();
    Ok((
        input,
        (
            Sensor {
                position: (s_x, s_y),
                man: manhattan(&(s_x, s_y), &(b_x, b_y)),
            },
            (b_x, b_y),
        ),
    ))
}

fn manhattan(i: &Ix, j: &Ix) -> isize {
    (i.0 - j.0).abs() + (i.1 - j.1).abs()
}

fn calc_y_covered(y: isize, sensor: &(Sensor, Ix)) -> Option<Range<isize>> {
    let y_diff = (y - sensor.0.position.1).abs();
    let d = sensor.0.man - y_diff;
    let x_min = sensor.0.position.0 - d;
    let x_max = sensor.0.position.0 + d + 1;
    if x_max > x_min {
        Some(x_min..x_max)
    } else {
        None
    }
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        input
    ))
    .unwrap()
    .trim()
    .to_owned();
    let sensors: Vec<_> = input.lines().map(|x| parse_input(x).unwrap().1).collect();
    let beacons: HashSet<Ix> = HashSet::from_iter(sensors.iter().map(|x| x.1));
    let diamonds: HashSet<_> = sensors.iter().map(|x| x.0.to_diamond()).collect();
    let mut y_covered: Vec<_> = sensors
        .iter()
        .filter_map(|x| calc_y_covered(2000000, x))
        .collect();
    y_covered.sort_by(|x, y| x.start.cmp(&y.start));
    let y_covered = reduce_sorted_range(y_covered.into_iter());
    let n = y_covered.iter().map(|x| x.len()).sum::<usize>()
        - beacons
            .iter()
            .filter(|&x| x.1 == 2000000 && y_covered.iter().any(|y| y.contains(&x.0)))
            .count();
    println!("day15a: {}", n);
    let mut search_area = HashSet::from([(Sensor {
        position: (2000000, 2000000),
        man: 4000000,
    })
    .to_diamond()]);
    for d in diamonds.into_iter() {
        let mut x = HashSet::new();
        for s in search_area.drain() {
            x.extend(s.subtract(&d));
        }
        search_area = x;
    }
    let n = search_area
        .into_iter()
        .map(calc_center)
        .find(|x| (0..4000000).contains(&x.0) && (0..4000000).contains(&x.1))
        .unwrap();
    println!("day15b: {}", n.0 * 4000000 + n.1);
}

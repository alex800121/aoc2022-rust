use project_root::get_project_root;
use nom::{ IResult, bytes::complete::{ tag, take_while } };

type Ix = (isize, isize);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    position: Ix,
    closest_beacon: Ix
}

fn parse_input(input: &str) -> IResult<&str, Sensor> {
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
    Ok((input, Sensor {
        position: (s_x, s_y),
        closest_beacon: (b_x, b_y)
    }))
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let sensors: Vec<_> = input.lines().map(|x| parse_input(x).unwrap()).collect();
    dbg!(sensors);
}

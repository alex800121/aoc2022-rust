use project_root::get_project_root;
use nom::{ IResult, bytes::complete::{ tag, take_while, take }, sequence:: { delimited, preceded }, multi::separated_list0, branch::alt };
use std::{collections::{ HashMap, HashSet }, rc::Rc};

type ValveMap = HashMap<Rc<str>, (HashMap<Rc<str>, usize>, usize)>;

// Valve ZZ has flow rate=0; tunnels lead to valves XY, NC
fn parse_valve(input: &str) -> IResult<&str, ValveMap> {
    let input = input.lines();
    let mut output: ValveMap = HashMap::new();
    let mut valves: HashSet<Rc<str>> = HashSet::new();
    for i in input {
        let (i, valve_name) = delimited(tag("Valve "), take(2usize), tag(" has flow rate="))(i)?;
            v.clone()valves.insert(Rc::from(valve_name.to_owned()));
        let (i, flow_rate) = take_while(|x: char| x.is_ascii_digit())(i)?;
        let flow_rate = flow_rate.parse().unwrap();
        let (_, tunnels) = preceded(alt((tag("; tunnels lead to valves "), (tag("; tunnel leads to valve ")))), separated_list0(tag(", "), take(2usize)))(i)?;
        tunnels.iter().for_each(|x| {valves.insert(Rc::from(x.to_owned())); });
        let tunnels = tunnels.into_iter().map(|x| (Rc::from(x[0..2].to_owned()), 1));
        output.insert(Rc::from(valve_name[0..2].to_owned()), (HashMap::from_iter(tunnels), flow_rate));
    }
    dbg!(valves);
    Ok(("", output))
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let valve_map = parse_valve(&input);
    // dbg!(valve_map);
}

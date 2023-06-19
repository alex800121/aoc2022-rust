use project_root::get_project_root;
use nom::{ IResult, bytes::complete::{ tag, take_while, take }, sequence:: { delimited, preceded }, multi::separated_list0, branch::alt };
use std::{collections::{HashMap, HashSet, BTreeSet}, rc::Rc};
use std::hash::Hash;
use aoc2022::bfs;

type ValveMap = HashMap<Rc<str>, (HashMap<Rc<str>, usize>, usize)>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct GameState {
    timer: usize,
    at_valve: Rc<str>,
    opened_valves: BTreeSet<Rc<str>>,
    closed_valves: BTreeSet<Rc<str>>,
}


fn nexts(valve_map: &ValveMap, (GameState { timer, at_valve, opened_valves, closed_valves }, pressure): &(GameState, usize), results: &mut HashMap<GameState, usize>) -> HashMap<GameState, usize> {
    let next_valves = &valve_map.get(at_valve).unwrap().0;
    let mut next_gamestates: HashMap<GameState, usize> = HashMap::new();
    for (next_valve, distance) in next_valves {
        if closed_valves.contains(next_valve) && *timer >= distance + 2 {
            let new_pressure = pressure + valve_map.get(next_valve).unwrap().1 * (timer - distance - 1);
            let mut new_closed_valves = closed_valves.clone();
            new_closed_valves.retain(|x| x != next_valve);
            let mut new_opened_valves = opened_valves.clone();
            new_opened_valves.insert(next_valve.clone());
            let new_gamestate = GameState {
                timer: timer - distance - 1,
                at_valve: next_valve.clone(),
                opened_valves: new_opened_valves,
                closed_valves: new_closed_valves,
            };
            if let Some(old_pressure) = results.get_mut(&new_gamestate) {
                if *old_pressure < new_pressure {
                    *old_pressure = new_pressure;
                    next_gamestates.insert(new_gamestate, new_pressure);
                }
            } else {
                next_gamestates.insert(new_gamestate.clone(), new_pressure);
                results.insert(new_gamestate, new_pressure);
            }
        }
    }
    next_gamestates
}

fn parse_valve(input: &str) -> IResult<&str, (ValveMap, GameState)> {
    let input = input.lines();
    let mut valve_map: HashMap<Rc<str>, HashMap<Rc<str>, usize>> = HashMap::new();
    let mut remained: ValveMap = HashMap::new();
    let mut starts = GameState {
        timer: 0,
        at_valve: Rc::from("AA"),
        opened_valves: BTreeSet::new(),
        closed_valves: BTreeSet::new(),
    };
    let mut valve_name_cache: HashSet<Rc<str>> = HashSet::new();
    for i in input {
        let (i, valve_name) = delimited(tag("Valve "), take(2usize), tag(" has flow rate="))(i)?;
        let valve_name = if let Some(v) = valve_name_cache.get(valve_name) {
            v.clone()
        } else {
            let v: Rc<str> = Rc::from(valve_name);
            valve_name_cache.insert(v.clone());
            v
        };
        let (i, flow_rate) = take_while(|x: char| x.is_ascii_digit())(i)?;
        let flow_rate = flow_rate.parse().unwrap();
        let (_, tunnels) = preceded(alt((tag("; tunnels lead to valves "), (tag("; tunnel leads to valve ")))), separated_list0(tag(", "), take(2usize)))(i)?;
        let tunnels = tunnels.iter().map(|&t| {
            if let Some(v) = valve_name_cache.get(t) {
                (v.clone(), 1)
            } else {
                let v: Rc<str> = Rc::from(t);
                valve_name_cache.insert(v.clone());
                (v, 1)
            }
        });
        if valve_name.as_ref() == "AA" {
            starts.at_valve = valve_name.clone();
            remained.insert(valve_name.clone(), (HashMap::new(), flow_rate));
        } else if flow_rate > 0 {
            starts.closed_valves.insert(valve_name.clone());
            remained.insert(valve_name.clone(), (HashMap::new(), flow_rate));
        }
        valve_map.insert(valve_name, HashMap::from_iter(tunnels));
    }
    let binding = remained.clone();
    let interest = binding.keys().map(|x| x.to_owned()).collect::<Vec<_>>();
    for (v, (tunnels, _)) in remained.iter_mut() {
        let mut acc = valve_map.get(v).unwrap().clone();
        let mut acc2 = HashMap::new();
        for i in acc.clone().drain() {
            if interest.contains(&i.0) {
                tunnels.insert(i.0, i.1);
            }
        }
        while !interest.iter().all(|x| x == v || tunnels.contains_key(x)) {
            for (v1, d1) in acc.iter() {
                for (v2, d2) in valve_map.get(v1).unwrap() {
                    if v2 != v && !tunnels.contains_key(v2) {
                        acc2.insert(v2.to_owned(), d1 + d2);
                    }
                }
            }
            acc = acc2.clone();
            for i in acc2.drain() {
                if interest.contains(&i.0) {
                    tunnels.insert(i.0, i.1);
                }
            }
        }
    }
    for (v1, m1) in remained.iter() {
        for (v2, d) in m1.0.iter() {
            assert_eq!(Some(d), remained.get(v2).and_then(|x| x.0.get(v1)));
        }
    }
    Ok(("", (remained, starts)))
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().to_owned();
    let (_, (valve_map, starts)) = parse_valve(&input).unwrap();
    let starts1 = HashMap::from([(GameState { timer: 30, ..starts.clone() }, 0)]);
    let starts2 = HashMap::from([(GameState { timer: 26, ..starts }, 0)]);
    let results1 = bfs(starts1, |x| x.is_empty(), |x, y| nexts(&valve_map, x, y));
    println!("day16a: {}", results1.iter().map(|x| x.1).max().unwrap());
    let results2 = bfs(starts2, |x| x.is_empty(), |x, y| nexts(&valve_map, x, y));
    let mut results2: Vec<_> = results2.into_iter().collect();
    results2.sort_by(|x, y| y.1.cmp(&x.1));
    let mut max = 0;
    let mut a = 0;
    let mut c = results2.len() - 1;
    while a < c {
        let mut b = a + 1;
        'b: while b < c {
            if results2[a].0.opened_valves.is_disjoint(&results2[b].0.opened_valves) {
                if max >= results2[a].1 + results2[b].1 {
                    break 'b;
                } else {
                    max = results2[a].1 + results2[b].1;
                    c = b;
                    break 'b;
                }
            } else {
                b += 1;
            }
        }
        a += 1;
    }
    println!("day16b: {}", max);
}

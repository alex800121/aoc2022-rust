use nom::{
    bytes::complete::{tag, take_till, take_until},
    sequence::tuple,
    IResult,
    character::streaming::space0,
    combinator::opt,
};
use project_root::get_project_root;
use std::hash::Hash;
use std::collections::{BTreeSet, BTreeMap, HashMap};

const DAYA: usize = 30;
const DAYB: usize = 26;

type ValveValue<'a> = BTreeMap<&'a str, usize>;
type ValveMapRaw<'a> = BTreeMap<&'a str, BTreeMap<&'a str, usize>>;
type ValveMap<'a> = HashMap<(&'a str, &'a str), usize>;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GameState<'a> {
    not_visited: ValveValue<'a>,
    current_pos: &'a str,
    count_down: usize
}
type GameStates<'a> = HashMap<GameState<'a>, usize>;

fn parse_input(input: &str) -> IResult<&str, (ValveValue<'_>, ValveMapRaw<'_>)> {
    let mut valve_value: ValveValue = BTreeMap::new();
    let mut valve_map: ValveMapRaw = BTreeMap::new();
    for i in input.split('\n') {
        let (i, (_, valve_name, _)) = tuple((tag("Valve "), take_till(|x: char| x.is_whitespace()), space0))(i)?;
        let (i, (_, flow_rate, _, _)) = tuple((tag("has flow rate="), take_until(";"), tag(";"), space0))(i)?;
        let flow_rate: usize = flow_rate.parse().unwrap();
        let (tunnels, _) = tuple((
            tag("tunnel"),
            opt(tag("s")),
            space0,
            tag("lead"),
            opt(tag("s")),
            space0,
            tag("to valve"),
            opt(tag("s")),
            space0
        ))(i)?;
        let tunnels: BTreeMap<&str, usize> = tunnels.split(", ").map(|x| (x, 1)).collect();
        if valve_name == "AA" || flow_rate > 0 {
            valve_value.insert(valve_name, flow_rate);
        }
        valve_map.insert(valve_name, tunnels);
    }
    Ok((input, (valve_value, valve_map)))
}

fn reduce_map<'a>(valve_value: &ValveValue<'a>, valve_map: &ValveMapRaw<'a>) -> ValveMap<'a> {
    let mut new_valve_map: ValveMapRaw = BTreeMap::new();
    for &valve in valve_value.keys() {
        let mut tunnels: BTreeMap<&str, usize> = BTreeMap::new();
        let mut starts: BTreeMap<&str, usize> = BTreeMap::from([(valve, 0)]);
        let mut visited: BTreeSet<&str> = BTreeSet::from([valve]);
        while tunnels.len() < valve_value.len() - 1 {
            let mut nexts: BTreeMap<&str, usize> = BTreeMap::new();
            for (start, current) in starts.into_iter() {
                if let Some(next_tunnels) = valve_map.get(start) {
                    for (&next_tunnel, d) in next_tunnels.iter() {
                        if !visited.contains(next_tunnel) {
                            nexts.insert(next_tunnel, current + d);
                            visited.insert(next_tunnel);
                            if let Some((&k, _)) = valve_value.get_key_value(next_tunnel) {
                                tunnels.insert(k, current + d);
                            }
                        }
                    }
                }
            }
            starts = nexts;
        }
        new_valve_map.insert(valve, tunnels);
    }
    let mut final_valve_map: ValveMap = HashMap::new();
    for (&v, m) in new_valve_map.iter() {
        for (&u, d) in m.iter() {
            assert_eq!(d, new_valve_map.get(u).unwrap().get(v).unwrap());
            final_valve_map.insert((v, u), *d);
            final_valve_map.insert((u, v), *d);
        }
    }
    final_valve_map
}

fn bfs<'a>(valve_map: &ValveMap<'a>, mut init_states: GameStates<'a>) -> GameStates<'a> {
    let mut final_states: GameStates = HashMap::new();
    while !init_states.is_empty() {
        let mut next_states: GameStates = HashMap::new();
        for (game_state, score) in init_states.drain() {
            let mut blocked = true;
            for (&tunnel, value) in game_state.not_visited.iter() {
                let distance = valve_map.get(&(tunnel, game_state.current_pos)).unwrap();
                if game_state.count_down > distance + 1 {
                    blocked = false;
                    let new_count_down = game_state.count_down - distance - 1;
                    next_states.insert(GameState {
                        not_visited: {
                            let mut not_visited = game_state.not_visited.clone();
                            not_visited.remove(tunnel);
                            not_visited
                        },
                        current_pos: tunnel,
                        count_down: new_count_down
                    }, score + (new_count_down * value));
                }
            }
            if blocked {
                if let Some(old_score) = final_states.get(&game_state) {
                    if score > *old_score {
                        final_states.insert(game_state, score);
                    }
                } else {
                    final_states.insert(game_state, score);
                }
            }
        }
        init_states = next_states;
    }
    final_states
}

pub fn run(input: usize) {
    let raw_input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        input
    ))
        .unwrap();
    let input = raw_input.trim();
    let (_, (mut valve_value, valve_map)) = parse_input(input).unwrap();
    let valve_map = reduce_map(&valve_value, &valve_map);
    let current_pos = valve_value.remove_entry("AA").unwrap().0;
    let init_states1: GameStates = HashMap::from([(GameState {
        not_visited: valve_value.clone(),
        current_pos,
        count_down: DAYA
    }, 0)]);
    let a = bfs(&valve_map, init_states1);
    println!("day16a: {}", a.iter().fold(0, |x, y| x.max(*y.1)));
    let init_states2: GameStates = HashMap::from([(GameState {
        not_visited: valve_value.clone(),
        current_pos,
        count_down: DAYB
    }, 0)]);
    let b = bfs(&valve_map, init_states2);
    let mut b = b.into_iter().map(|x| (x.1, {
        let visited = valve_value.clone();
        visited.into_keys().filter(|&y| !x.0.not_visited.contains_key(y)).collect::<BTreeSet<_>>()
    })).collect::<Vec<_>>();
    b.sort_by(|x, y| y.0.cmp(&x.0));
    let mut i = 0;
    let mut j;
    let mut k = b.len();
    let mut m = 0;
    while i < k {
        j = i + 1;
        while j < k {
            let x = b.get(i).unwrap();
            let y = b.get(j).unwrap();
            if x.1.is_disjoint(&y.1) && m < x.0 + y.0 {
                m = x.0 + y.0;
                k = j;
            }
            j += 1;
        }
        i += 1;
    }
    println!("day16b: {}", m);
}

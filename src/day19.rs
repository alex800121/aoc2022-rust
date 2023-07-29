use std::{collections::{HashSet, HashMap}, ops::Index, array::from_fn};
use std::io::Write;

use aoc2022::ZipWith;
use nom::{
    IResult,
    sequence::tuple,
    bytes::complete::{tag, take_until}
};
use num::Integer;
use project_root::get_project_root;

type Rocks = [isize; 3];
type Robots = [isize; 4];
type RobotCosts = [Rocks; 4];
type Geode = isize;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    number: isize,
    costs: RobotCosts,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GameState {
    rocks: Rocks,
    robots: Robots,
    max_robots: Robots,
    timer: isize,
}

impl GameState {
    fn default(blueprint: &Blueprint, timer: isize) -> Self {
        let max_robots = blueprint.costs.iter().fold([0, 0, 0, timer], |acc, &x| [
            acc[0].max(x[0]),
            acc[1].max(x[1]),
            acc[2].max(x[2]),
            timer
        ]);
        GameState { 
            rocks: [0; 3],
            robots: [1, 0, 0, 0],
            max_robots,
            timer,
        }
    }

    fn bfs(self, blueprint: &Blueprint) -> HashMap<GameState, Geode> {
        let mut init_state = HashMap::from([(self, 0)]);
        let mut done: HashMap<GameState, Geode> = HashMap::new();
        let mut cache: HashMap<GameState, Geode> = init_state.clone();
        while !init_state.is_empty() {
            dbg!(init_state.len());
            let mut next_state = HashMap::new();
            for (state, geode) in init_state.drain() {
                next_state.extend(state.next(geode, blueprint, &mut done, &mut cache));
            }
            init_state = next_state;
        }
        done
    }

    fn next(self, geode: isize, blueprint: &Blueprint, done: &mut HashMap<GameState, Geode>, cache: &mut HashMap<GameState, Geode>) -> HashMap<GameState, Geode> {
        todo!()
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, (
        _,
        n,
        _,
        ore_ore,
        _,
        clay_ore,
        _,
        obsidian_ore,
        _,
        obsidian_clay,
        _,
        geode_ore,
        _,
        geode_obsidian
    )) = tuple((
        tag("Blueprint "),
        take_until(":"),
        tag(": Each ore robot costs "),
        take_until(" "),
        tag(" ore. Each clay robot costs "),
        take_until(" "),
        tag(" ore. Each obsidian robot costs "),
        take_until(" "),
        tag(" ore and "),
        take_until(" "),
        tag(" clay. Each geode robot costs "),
        take_until(" "),
        tag(" ore and "),
        take_until(" "),
    ))(input)?;
    let n: isize = n.parse().unwrap();
    let ore_ore: isize = ore_ore.parse().unwrap();
    let clay_ore: isize = clay_ore.parse().unwrap();
    let obsidian_ore: isize = obsidian_ore.parse().unwrap();
    let obsidian_clay: isize = obsidian_clay.parse().unwrap();
    let geode_ore: isize = geode_ore.parse().unwrap();
    let geode_obsidian: isize = geode_obsidian.parse().unwrap();
    Ok((input, Blueprint {
        number: n,
        costs: [
            [ore_ore, 0, 0],
            [clay_ore, 0, 0],
            [obsidian_ore, obsidian_clay, 0],
            [geode_ore, 0, geode_obsidian]
        ],
    }))
}

pub fn run(input: usize) {
    std::fs::File::create("log19");
    let blueprints = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().lines().map(|x| parse_blueprint(x).unwrap().1).collect::<Vec<_>>();
    // let blueprints = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().lines().map(|x| parse_blueprint(x).unwrap().1).collect::<Vec<_>>();
    // let init_states_a = blueprints.iter().map(|x| (GameState::default(x, 24), x));
    let init_states_a = blueprints.iter().map(|x| (GameState::default(x, 24), x));
    let ans_a = init_states_a.map(|(init_state, blueprint)| {
        let ans = init_state.bfs(blueprint);
        dbg!(*ans.values().max().unwrap());
        *ans.values().max().unwrap() * blueprint.number
    }).sum::<isize>();
    dbg!(ans_a);
    // let init_states_b = blueprints.iter().map(|x| (GameState::default(x, 30), x)).take(3);
}

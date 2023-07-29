use std::{collections::BTreeMap, array::from_fn};

use nom::{
    IResult,
    sequence::tuple,
    bytes::complete::{tag, take_until}
};
use project_root::get_project_root;

type Rocks = [isize; 3];
type Robots = [isize; 4];
type RobotCosts = [Rocks; 4];

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

    fn bfs(self, blueprint: &Blueprint) -> isize {
        let mut init_state = BTreeMap::from([(self, 0)]);
        let mut done: isize = 0;
        while !init_state.is_empty() {
            let mut next_state = BTreeMap::new();
            for (state, geode) in init_state.iter() {
                let potential = (((state.timer - 1 + (2 * state.robots[3])) * state.timer) / 2) + geode;
                if potential <= done { continue; };
                let new = geode + (state.timer * state.robots[3]);
                done = done.max(new);
                let mut geode_need_time = Some(0);
                for (i, rocks) in blueprint.costs[3].iter().enumerate() {
                    let need = 0.max(rocks - state.rocks[i]);
                    let robot = state.robots[i];
                    match (need, robot) {
                        (need, robot) if robot <= 0 && need > 0 => {
                            geode_need_time = None;
                        },
                        (_, robot) if robot > 0 => {
                            geode_need_time = geode_need_time.map(|x| x.max(num::Integer::div_ceil(&need, &robot)));
                        }
                        _ => {},
                    }
                }
                match geode_need_time {
                    Some(geode_need_time) if geode_need_time + 1 < state.timer => {
                        let g = GameState {
                            rocks: {
                                from_fn(|i| {
                                    state.rocks[i] + ((geode_need_time + 1) * state.robots[i]) - blueprint.costs[3][i]
                                })
                            },
                            robots: {
                                from_fn(|i| {
                                    if i == 3 {
                                        state.robots[i] + 1
                                    } else {
                                        state.robots[i]
                                    }
                                })
                            },
                            timer: state.timer - geode_need_time - 1,
                            ..*state
                        };
                        let new = geode + ((geode_need_time + 1) * state.robots[3]);
                        next_state.insert(g, new);
                    },
                    _ => {}
                }
                for j in 0..3 {
                    let cost = blueprint.costs[j];
                    if state.robots[j] >= state.max_robots[j] { continue; }
                    let mut need_time = Some(0);
                    for (i, rocks) in cost.iter().enumerate() {
                        let need = 0.max(rocks - state.rocks[i]);
                        let robot = state.robots[i];
                        match (need, robot, &geode_need_time) {
                            (need, robot, _) if need > 0 && robot <= 0 => {
                                need_time = None;
                            },
                            (need, robot, &Some(geode_need_time)) if robot > 0 => {
                                let time = num::Integer::div_ceil(&need, &robot);
                                if time >= geode_need_time {
                                    need_time = None;
                                } else {
                                    need_time = need_time.map(|x| x.max(time));
                                }
                            },
                            (need, robot, &None) if robot > 0 => {
                                need_time = need_time.map(|x| x.max(num::Integer::div_ceil(&need, &robot)));
                            },
                            _ => {}
                        }
                    }
                    match need_time {
                        Some(need_time) if need_time + 1 < state.timer => {
                            let g = GameState {
                                rocks: {
                                    from_fn(|i| {
                                        state.rocks[i] + ((need_time + 1) * state.robots[i]) - blueprint.costs[j][i]
                                    })
                                },
                                robots: {
                                    from_fn(|i| {
                                        if i == j {
                                            state.robots[i] + 1
                                        } else {
                                            state.robots[i]
                                        }
                                    })
                                },
                                timer: state.timer - need_time - 1,
                                ..*state
                            };
                            let new = geode + ((need_time + 1) * state.robots[3]);
                            next_state.insert(g, new);
                        },
                        _ => {}
                    }
                }
            }
            init_state = next_state;
        }
        done
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
    // let blueprints = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().lines().map(|x| parse_blueprint(x).unwrap().1).collect::<Vec<_>>();
    let blueprints = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap().trim().lines().map(|x| parse_blueprint(x).unwrap().1).collect::<Vec<_>>();
    let init_states = blueprints.iter().map(|x| (GameState::default(x, 24), x));
    let mut ans_a = 0;
    for (game_state, blueprint) in init_states {
        let i = game_state.bfs(blueprint);
        ans_a += i * blueprint.number;
    }
    println!("day19a: {}", ans_a);
    let init_states = blueprints.iter().map(|x| (GameState::default(x, 32), x)).take(3);
    let mut ans_a = 1;
    for (game_state, blueprint) in init_states {
        let i = game_state.bfs(blueprint);
        ans_a *= i;
    }
    println!("day19b: {}", ans_a);
}

use std::{collections::HashMap, slice::Iter, rc::Rc};
use nom::{IResult, multi, bytes::complete, character, branch, combinator};
use project_root::get_project_root;

type Tree = HashMap<Rc<str>, FS>;
type Commands = Vec<Command>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Command {
    Parent,
    CD(Rc<str>),
    LS(Vec<Entry>),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Entry {
    Dir { name: Rc<str>, },
    File { name: Rc<str>, size: usize },
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum FS {
    Dir { children: Tree, },
    File { size: usize, },
}

fn parse_commands(input: &str) -> IResult<&str, Commands> {
    multi::many0(branch::alt((parse_parent, parse_cd, parse_ls)))(input)
}

fn parse_parent(input: &str) -> IResult<&str, Command> {
    let (input, _) = complete::tag("$ cd ..")(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Command::Parent))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = complete::tag("$ cd ")(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Command::CD(name.into())))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = complete::tag("$ ls")(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    let (input, entries) = complete::take_till(|x| x == '$')(input)?;
    // println!("{:?}", entries);
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    let (_, entries) = multi::many0(parse_entry)(entries)?;
    Ok((input, Command::LS(entries)))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, entry) = branch::alt((parse_dir, parse_file))(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, entry))
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    let (input, size) = complete::take_while1(|x: char| x.is_ascii_digit())(input)?;
    let size = size.parse().unwrap();
    let (input, _) = combinator::opt(character::complete::space0)(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Entry::File{ name: name.into(), size,}))
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
    let (input, _) = complete::tag("dir")(input)?;
    let (input, _) = combinator::opt(character::complete::space0)(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Entry::Dir { name: name.into() }))
}

fn build_tree(commands: Iter<Command>) -> Tree {
    let mut queue: Vec<(Rc<str>, Tree)> = Vec::new();
    let mut tree: Tree = HashMap::new();
    for command in commands {
        match command {
            Command::LS(entries) => {
                entries.iter().for_each(|x|
                    match x {
                        Entry::File{ name, size, } => {
                            tree.insert(name.clone(), FS::File{ size: *size, });
                            // println!("inserting file: {:?}", x);
                        },
                        Entry::Dir { name } => {
                            tree.insert(name.clone(), FS::Dir{ children: HashMap::new(), });
                            // println!("inserting dir: {:?}", x);
                        },
                    }
                );
            },
            Command::Parent => { 
                if let Some((name, mut parent)) = queue.pop() {
                    parent.insert(name, FS::Dir { children: tree });
                    tree = parent;
                }
            },
            Command::CD(name) => { 
                queue.push((name.clone(), tree.drain().collect()));
            },
        }
    }
    while let Some((name, mut parent)) = queue.pop() {
        parent.insert(name, FS::Dir { children: tree });
        tree = parent;
    }
    tree
}

fn calc_dir_size(tree: &Tree, acc: &mut Vec<usize>) -> usize {
    let mut s = 0;
    tree.iter().for_each(|x|
        match x.1 {
            FS::File { size } => { s += size },
            FS::Dir { children } => { 
                s += calc_dir_size(children, acc);
            },
        }
    );
    acc.push(s);
    s
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let (_, commands) = parse_commands(&input).unwrap();
    let commands = commands.iter();
    let tree = build_tree(commands);
    let mut acc = Vec::new();
    let root_size = calc_dir_size(&tree, &mut acc);
    acc.sort();
    println!("day7a: {}", &acc.iter().filter(|&&x| x <= 100000).sum::<usize>());
    let max_size = 70000000 - 30000000;
    let need_free = root_size - max_size;
    println!("day7b: {}", acc.into_iter().find(|&x| x >= need_free).unwrap());
}

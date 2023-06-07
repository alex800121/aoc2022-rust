use std::{collections::HashMap, slice::IterMut};
use nom::{IResult, multi, bytes::complete, character, branch, combinator};
use project_root::get_project_root;

type Tree = HashMap<String, FS>;
type Commands<'a> = Vec<Command<'a>>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Command<'a> {
    Parent,
    CD(&'a str),
    LS(Vec<Entry<'a>>),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Entry<'a> {
    Dir { name: &'a str, },
    File { name: &'a str, size: usize },
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum FS {
    Dir { children: Tree, },
    File { size: usize, },
}

fn parse_commands(input: &str) -> IResult<&str, Commands<'_>> {
    multi::many0(branch::alt((parse_parent, parse_cd, parse_ls)))(input)
}

fn parse_parent(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = complete::tag("$ cd ..")(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Command::Parent))
}

fn parse_cd(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = complete::tag("$ cd ")(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Command::CD(name)))
}

fn parse_ls(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = complete::tag("$ ls")(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    let (input, entries) = complete::take_till(|x| x == '$')(input)?;
    // println!("{:?}", entries);
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    let (_, entries) = multi::many0(parse_entry)(entries)?;
    Ok((input, Command::LS(entries)))
}

fn parse_entry(input: &str) -> IResult<&str, Entry<'_>> {
    let (input, entry) = branch::alt((parse_dir, parse_file))(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, entry))
}

fn parse_file(input: &str) -> IResult<&str, Entry<'_>> {
    let (input, size) = complete::take_while1(|x: char| x.is_ascii_digit())(input)?;
    let size = size.parse().unwrap();
    let (input, _) = combinator::opt(character::complete::space0)(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Entry::File{name, size,}))
}

fn parse_dir(input: &str) -> IResult<&str, Entry<'_>> {
    let (input, _) = complete::tag("dir")(input)?;
    let (input, _) = combinator::opt(character::complete::space0)(input)?;
    let (input, name) = character::complete::not_line_ending(input)?;
    let (input, _) = combinator::opt(character::complete::newline)(input)?;
    Ok((input, Entry::Dir { name }))
}

fn build_tree<'a>(commands: &mut IterMut<'a, Command<'a>>, tree: &mut Tree) {
    while let Some(command) = commands.next() {
        match command {
            Command::LS(entries) => {
                entries.iter_mut().for_each(|x|
                    match x {
                        Entry::File{ name, size, } => {
                            tree.insert(name.to_owned(), FS::File{ size: *size, });
                            // println!("inserting file: {:?}", x);
                        },
                        Entry::Dir { name } => {
                            tree.insert(name.to_owned(), FS::Dir{ children: HashMap::new(), });
                            // println!("inserting dir: {:?}", x);
                        },
                    }
                );
            },
            Command::Parent => { 
                return;
            },
            Command::CD(name) => { 
                if let Some(entry) = tree.get_mut(&name.to_owned()) {
                    // println!("cd into entry {} {:?}", name, entry);
                    match entry {
                        FS::File{ .. } => panic!("cd into a file {} {:?}", name, entry),
                        FS::Dir { children } => {
                            build_tree(commands, children);
                        },
                    }
                } else {
                    // println!("inserting and cd into dir: {:?}", name);
                    tree.insert( name.to_owned(), FS::Dir { children: HashMap::new(), });
                }
            },
        }
    }
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
    let (_, mut commands) = parse_commands(&input).unwrap();
    let mut tree: Tree = HashMap::new();
    let mut commands = commands.iter_mut();
    build_tree(&mut commands, &mut tree);
    let mut acc = Vec::new();
    let root_size = calc_dir_size(&tree, &mut acc);
    acc.sort();
    println!("day7a: {}", &acc.iter().filter(|&&x| x <= 100000).sum::<usize>());
    let max_size = 70000000 - 30000000;
    let need_free = root_size - max_size;
    println!("day7b: {}", acc.into_iter().find(|&x| x >= need_free).unwrap());
}

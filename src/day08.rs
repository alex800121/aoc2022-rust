use aoc2022::Transpose;
use aoc2022::zip_with;
use project_root::get_project_root;
use std::collections::HashSet;

fn visible(input: &mut Vec<Vec<isize>>, acc: &mut HashSet<(usize, usize)>) {
    let mut max_height;
    for (i, row) in input.iter().enumerate() {
        max_height = -1;
        for (j, cell) in row.iter().enumerate() {
            if *cell > max_height {
                acc.insert((j, i));
                max_height = *cell;
            }
        }
        max_height = -1;
        for (j, cell) in row.iter().enumerate().rev() {
            if *cell > max_height {
                acc.insert((j, i));
                max_height = *cell;
            }
        }
    }
    let transposed = input.clone().transpose();
    for (i, row) in transposed.iter().enumerate() {
        max_height = -1;
        for (j, cell) in row.iter().enumerate() {
            if *cell > max_height {
                acc.insert((i, j));
                max_height = *cell;
            }
        }
        max_height = -1;
        for (j, cell) in row.iter().enumerate().rev() {
            if *cell > max_height {
                acc.insert((i, j));
                max_height = *cell;
            }
        }
    }
}

fn row_scenic_score(input: &[isize]) -> Vec<usize> {
    let mut v = Vec::new();
    for (i, cell1) in input.iter().enumerate() {
        let mut j = i + 1;
        let mut acc = 0;
        while let Some(cell2) = input.get(j) {
            if cell1 <= cell2 {
                acc += 1;
                break;
            } else {
                acc += 1;
                j += 1;
            }
        }
        v.push(acc);
    }
    v
}

fn scenic_score(input: &mut Vec<Vec<isize>>) -> Vec<Vec<usize>> {
    let s1 = input.iter().map(|x| row_scenic_score(x)).collect::<Vec<_>>();
    input.iter_mut().for_each(|x| x.reverse());
    let mut s2 = input.iter().map(|x| row_scenic_score(x)).collect::<Vec<_>>();
    s2.iter_mut().for_each(|x| x.reverse());
    input.iter_mut().for_each(|x| x.reverse());
    let mut input = input.clone().transpose();
    let mut s3 = input.iter().map(|x| row_scenic_score(x)).collect::<Vec<_>>();
    let s3 = s3.transpose();
    input.iter_mut().for_each(|x| x.reverse());
    let mut s4 = input.iter().map(|x| row_scenic_score(x)).collect::<Vec<_>>();
    s4.iter_mut().for_each(|x| x.reverse());
    let s4 = s4.transpose();
    let s1 = zip_with(s1, s2, |a, b| zip_with(a, b, |x, y| x * y));
    let s1 = zip_with(s1, s3, |a, b| zip_with(a, b, |x, y| x * y));
    zip_with(s1, s4, |a, b| zip_with(a, b, |x, y| x * y))
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let mut input: Vec<_> = input.lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as isize).collect::<Vec<isize>>()).collect();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    visible(&mut input, &mut set);
    println!("day8a: {}", set.len());
    println!("day8b: {}", scenic_score(&mut input).iter().map(|x| x.iter().max().unwrap()).max().unwrap());
}

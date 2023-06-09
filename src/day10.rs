use project_root::get_project_root;

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}

fn read_instruction(register: &mut isize, acc: &mut Vec<isize>, instruction: &Instruction) {
    acc.push(*register);
    if let Instruction::Addx(n) = instruction {
        acc.push(*register);
        *register += n;
    }
}

pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let instructions = input.lines().map(|x| {
        let mut x = x.split(' ');
        match x.next() {
            Some("addx") => {
                let n = x.next().unwrap().parse().unwrap();
                Instruction::Addx(n)
            }
            _ => Instruction::Noop,
        }
    });
    let mut register = 1;
    let mut acc = Vec::new();
    // dbg!(instructions.collect::<Vec<_>>());
    instructions.for_each(|instruction| read_instruction(&mut register, &mut acc, &instruction));
    println!("day10a: {}", [19, 59, 99, 139, 179, 219].iter().fold(0, |x, y| x + (*y + 1) as isize * acc[*y]));
    println!("day10b: ");
    (0..240).map(|x| {
        let y = acc[x];
        let range = (y - 1)..=(y + 1);
        if range.contains(&(x as isize % 40)) {
            '#'
        } else {
            '.'
        }
    }).collect::<Vec<_>>().chunks(40).for_each(|x| {
        x.iter().for_each(|y| print!("{}", y));
        println!();
    });
}

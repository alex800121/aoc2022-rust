use project_root::get_project_root;

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}

fn read_instruction(register: &mut isize, acc: &mut Vec<isize>, instruction: &Instruction) {
    acc.push(*register);
    if let Instruction::Addx(n) = instruction {
        *register += n;
        acc.push(*register);
    }
}

pub fn run(input: usize) {
    // let input = std::fs::read_to_string(format!("{}/input/input{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
    let input = std::fs::read_to_string(format!("{}/input/test{:02}.txt", get_project_root().unwrap().to_str().unwrap(), input)).unwrap();
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
    dbg!((acc[18], acc[58], acc[88], acc[138], acc[178], acc[218]));
}

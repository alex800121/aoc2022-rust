fn fully_contains(list: &[usize]) -> bool {
    ((list[0]..=list[1]).contains(&list[2]) && (list[0]..=list[1]).contains(&list[3])) ||
    ((list[2]..=list[3]).contains(&list[0]) && (list[2]..=list[3]).contains(&list[1]))
}
fn overlaps(list: &[usize]) -> bool {
    ((list[0]..=list[1]).contains(&list[2]) || (list[0]..=list[1]).contains(&list[3])) ||
    ((list[2]..=list[3]).contains(&list[0]) || (list[2]..=list[3]).contains(&list[1]))
}
pub fn run(input: usize) {
    let input = std::fs::read_to_string(format!("../input/input{:02}.txt", input)).unwrap();
    let day4: Vec<_> = 
        input.lines()
            .map(|x| x.split(&['-', ','])
            .map(|y| y.parse::<usize>()
                    .unwrap())
            .collect::<Vec<_>>())
            .collect();
    let mut day4count = 0;
    for i in day4.iter() {
        if fully_contains(i){
            day4count += 1;
        }
    }
    println!("day4a: {:?}", day4count);
    day4count = 0;
    for i in day4 {
        if overlaps(&i) {
            day4count += 1;
        }
    }
    println!("day4b: {:?}", day4count);
}

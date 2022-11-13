use std::collections::HashSet;

#[aoc::main(08)]
fn main(input: &str) -> (i32, u32) {
    let instrs = get_instr(input);

    let mut executed: HashSet<i32> = HashSet::new();
    let mut next = 0;
    let mut p1 = 0;
    while !executed.contains(&next) {
        executed.insert(next);

        let (instr, arg) = &instrs[(next as usize)];
        match instr.as_str() {
            "jmp" => next += arg,
            "nop" => next += 1,
            "acc" => {
                p1 += arg;
                next += 1;
            }
            _ => panic!("Incorrect instruction"),
        }
    }
    println!("{}", p1);

    (p1, 0)
}

fn get_instr(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|row| {
            let (instr, arg) = row.split_once(" ").unwrap();
            (instr.to_string(), arg.parse::<i32>().unwrap())
        })
        .collect()
}

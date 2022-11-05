use std::fs;

const INPUT_FILE: &str = "input.txt";
const SUM_TO: i32 = 2020;

fn main() {
    let input = get_input();
    let res = find_sums(input);
    println!("{}", res);
}

//This solves part two
fn find_sums(v: Vec<i32>) -> i32 {
    for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == SUM_TO {
                    return i * j * k;
                }
            }
        }
    }
    return 0;
}

fn get_input() -> Vec<i32> {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i32>().expect("Not a number"))
        .collect()
}

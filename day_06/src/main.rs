use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

const INPUT_FILE: &str = "input.txt";
type Group = Vec<Vec<char>>;

fn main() {
    let groups = get_input();

    let mut sum_one = 0;
    for group in &groups {
        let ans: Vec<&char> = group.iter().flatten().collect();
        let set: HashSet<&char> = HashSet::from_iter(ans);
        sum_one += set.len();
    }
    println!("Part 1: {}", sum_one);

    let mut sum_two = 0;
    for group in &groups {
        let group_size = group.len();
        let ans: Vec<&char> = group.iter().flatten().collect();

        let mut counts: HashMap<char, usize> = HashMap::new();
        for x in ans {
            *counts.entry(*x).or_default() += 1;
        }

        for (_, n) in counts {
            if n == group_size {
                sum_two += 1;
            }
        }
    }
    println!("Part 2: {}", sum_two);
}

fn get_input() -> Vec<Group> {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|p| p.chars().collect())
                .collect()
        })
        .collect()
}

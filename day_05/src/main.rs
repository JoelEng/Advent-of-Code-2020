use std::cmp;
use std::fs;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let input = get_input();

    let mut highest = 0;
    let mut all_ids: Vec<i32> = vec![];
    for v in input {
        let (row, col) = v.split_at(7);
        let row_seat = bin_search(row.to_vec(), 128);
        let col_seat = bin_search(col.to_vec(), 8);
        let seat_id = row_seat * 8 + col_seat;
        all_ids.push(seat_id);
        highest = cmp::max(highest, seat_id);
    }
    println!("Part 1: {}", highest);

    for i in 0..1032 {
        if !all_ids.contains(&i) && all_ids.contains(&(i + 1)) && all_ids.contains(&(i - 1)) {
            println!("Part 2: {}", i);
        }
    }
}

fn bin_search(v: Vec<char>, length: i32) -> i32 {
    let mut from = 0;
    let mut to = length;
    for c in v {
        let size_change = (to - from) / 2;
        match c {
            'F' | 'L' => to -= size_change,
            'B' | 'R' => from += size_change,
            _ => println!("Something went wrong"),
        }
    }
    from
}

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    input
        .split_whitespace()
        .map(|r| r.chars().collect())
        .collect()
}

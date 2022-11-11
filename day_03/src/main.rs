use std::fs;

const INPUT_FILE: &str = "input.txt";
type Field = Vec<Vec<bool>>;

fn main() {
    let v = get_input();

    let mut prod = check_slope(&v, 3, 1);
    prod *= check_slope(&v, 1, 1);
    prod *= check_slope(&v, 5, 1);
    prod *= check_slope(&v, 7, 1);
    prod *= check_slope(&v, 1, 2);

    println!("{}", prod);
}

fn check_slope(v: &Field, col_change: usize, row_change: usize) -> i64 {
    let mut hit_trees = 0;
    let v_width = v[0].len();
    let mut col = 0;
    let mut row = 0;
    while row < v.len() {
        if v[row][col] {
            hit_trees += 1
        }
        col += col_change;
        if col >= v_width {
            col -= v_width;
        }
        row += row_change;
    }
    hit_trees
}

fn get_input() -> Field {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    input
        .split("\n")
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

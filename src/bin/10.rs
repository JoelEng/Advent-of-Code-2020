use itertools::Itertools;

#[aoc::main(10)]
fn main(input: &str) -> (i32, i32) {
    let mut joltage: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    joltage.push(0);
    joltage.sort();
    joltage.push(joltage[joltage.len() - 1] + 3);

    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for (a, b) in joltage.iter().tuple_windows() {
        let diff = b - a;
        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1;
        }
    }

    joltage.pop(); // the device always has to connect to the last adapter, so this doesn't change the count

    (diff_1 * diff_3, 0)
}

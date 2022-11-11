use regex::Regex;
use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "input.txt";
type Passport = HashMap<String, String>;
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let input = get_input();
    let sum = input.iter().filter(|p| check_passport(p).is_some()).count();
    println!("{}", sum);
}

fn check_passport(p: &Passport) -> Option<bool> {
    let byr = p.get("byr")?.parse::<i32>().ok()?;
    let iyr = p.get("iyr")?.parse::<i32>().ok()?;
    let eyr = p.get("eyr")?.parse::<i32>().ok()?;
    let hgt = p.get("hgt")?;
    let hcl = p.get("hcl")?.split("#").nth(1)?;
    let ecl = p.get("ecl")?.as_str();
    let pid = p.get("pid")?;

    check_hgt(hgt)?;
    i32::from_str_radix(hcl, 16).ok()?;
    (p.len() >= 7
        && byr >= 1910
        && byr <= 2002
        && iyr >= 2010
        && iyr <= 2020
        && eyr >= 2020
        && eyr <= 2030
        && pid.len() == 9
        && EYE_COLORS.contains(&ecl))
    .then_some(true)
}

fn check_hgt(hgt: &str) -> Option<bool> {
    let re = Regex::new(r"\pN+").ok()?;
    let n = re.find(hgt)?.as_str().parse::<i32>().ok()?;
    let re = Regex::new(r"\pL+").ok()?;
    let unit = re.find(hgt)?.as_str();

    ((unit == "cm" && n >= 150 && n <= 193) || (unit == "in" && n >= 59 && n <= 76)).then_some(true)
}

fn get_input() -> Vec<Passport> {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    let mut v: Vec<Passport> = vec![];
    for s in input.split("\n\n") {
        let mut map: HashMap<String, String> = HashMap::new();
        for entry in s.split_whitespace() {
            let k = entry.split(":").nth(0).unwrap().to_string();
            let v = entry.split(":").nth(1).unwrap().to_string();
            if k != "cid" {
                map.insert(k, v);
            }
        }
        v.push(map);
    }
    v
}

use std::collections::HashSet;
use std::fs::read_to_string;

pub fn count_valid_passports() -> Result<usize, &'static str> {
    let file_content = read_to_string("day4/data.txt").unwrap();
    let passports: Vec<&str> = file_content.split("\n\n").collect();
    let mut valid_passports = 0;
    for passport in passports {
        let flat_passport: HashSet<&str> = passport
            .split("\n")
            .map(|passport_part| passport_part.split(" ").collect::<Vec<&str>>())
            .flatten()
            .filter(|field| field.contains(':'))
            .collect();
        if validate_passport(flat_passport) {
            valid_passports += 1;
        }
    }
    return Ok(valid_passports);
}

fn validate_passport(passport: HashSet<&str>) -> bool {
    let required_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let passport_keys = passport
        .iter()
        .map(|field| *field.split(':').collect::<Vec<&str>>().get(0).unwrap())
        .collect();
    if (&required_keys - &passport_keys).len() != 0 {
        return false;
    }
    for field in passport {
        let field: Vec<&str> = field.split(':').collect();
        let key = *field.get(0).unwrap();
        let value = *field.get(1).unwrap();
        match key {
            "byr" => {
                if 1920 > value.parse().unwrap() || 2002 < value.parse().unwrap() {
                    return false;
                }
            }
            "iyr" => {
                if 2010 > value.parse().unwrap() || 2020 < value.parse().unwrap() {
                    return false;
                }
            }
            "eyr" => {
                if 2020 > value.parse().unwrap() || 2030 < value.parse().unwrap() {
                    return false;
                }
            }
            "hgt" => {
                if value.ends_with("cm") || value.ends_with("in") {
                    let height = value.get(0..value.len() - 2).unwrap().parse().unwrap();
                    let unit = value.get(value.len() - 2..value.len()).unwrap();
                    match unit {
                        "cm" => {
                            if 150 > height || height > 193 {
                                return false;
                            }
                        }
                        "in" => {
                            if 59 > height || height > 76 {
                                return false;
                            }
                        }
                        _ => (),
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                if value.chars().nth(0).unwrap() != '#'
                    || value.len() != 7
                    || value.get(1..).unwrap().chars().any(|c| !c.is_digit(16))
                {
                    return false;
                }
            }
            "ecl" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 || value.chars().any(|c| !c.is_digit(10)) {
                    return false;
                }
            }
            "cid" => (),
            _ => eprintln!("Unmatched value = {}", value),
        }
    }
    return true;
}

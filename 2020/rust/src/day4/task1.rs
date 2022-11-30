use std::collections::HashSet;
use std::fs::read_to_string;

pub fn count_valid_passports() -> Result<usize, &'static str> {
    let file_content = read_to_string("day4/data.txt").unwrap();
    let passports: Vec<&str> = file_content.split("\n\n").collect();
    let required_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let mut valid_passports = 0;
    for passport in passports {
        let passport_keys: HashSet<&str> = passport
            .split("\n")
            .map(|passport_part| passport_part.split(" ").collect::<Vec<&str>>())
            .flatten()
            .filter(|field| field.contains(':'))
            .map(|field| *field.split(':').collect::<Vec<&str>>().get(0).unwrap())
            .collect();
        if (&required_keys - &passport_keys).len() == 0 {
            valid_passports += 1;
        }
    }
    return Ok(valid_passports);
}

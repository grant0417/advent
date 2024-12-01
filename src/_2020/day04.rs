use std::fmt::Display;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn parse(s: &str) -> Passport {
        let mut passport = Passport::default();
        let fields = s.trim().split_whitespace();
        for field in fields {
            let (key, value) = field.split_once(':').unwrap();

            match key {
                "byr" => passport.byr = Some(value.into()),
                "iyr" => passport.iyr = Some(value.into()),
                "eyr" => passport.eyr = Some(value.into()),
                "hgt" => passport.hgt = Some(value.into()),
                "hcl" => passport.hcl = Some(value.into()),
                "ecl" => passport.ecl = Some(value.into()),
                "pid" => passport.pid = Some(value.into()),
                "cid" => passport.cid = Some(value.into()),
                _ => panic!("unknown key {key}:{value}"),
            }
        }
        passport
    }

    fn is_valid_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            Some(s) => {
                if let Some(s) = s.strip_suffix("cm") {
                    let Ok(hgt) = s.parse::<u32>() else {
                        return false;
                    };
                    hgt >= 150 && hgt <= 193
                } else if let Some(s) = s.strip_suffix("in") {
                    let Ok(hgt) = s.parse::<u32>() else {
                        return false;
                    };
                    hgt >= 59 && hgt <= 76
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn hcl_valid(&self) -> bool {
        match &self.hcl {
            Some(s) => {
                let mut chars = s.chars();
                chars.next() == Some('#') && chars.all(|c| c.is_ascii_hexdigit())
            }
            None => false,
        }
    }

    fn ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(s) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**s),
            None => false,
        }
    }

    fn pid_valid(&self) -> bool {
        match &self.pid {
            Some(s) => s.chars().all(|s| s.is_ascii_digit()) && s.len() == 9,
            None => false,
        }
    }

    fn is_valid_2(&self) -> bool {
        validate_year(&self.byr, 1920, 2002)
            && validate_year(&self.iyr, 2010, 2020)
            && validate_year(&self.eyr, 2020, 2030)
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }
}

fn validate_year(s: &Option<String>, min: u32, max: u32) -> bool {
    match s {
        Some(s) => match s.parse::<u32>() {
            Ok(i) => i >= min && i <= max,
            Err(_) => false,
        },
        None => false,
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::parse).collect()
}

pub fn part1(input: &str) -> impl Display {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.is_valid_1()).count()
}

pub fn part2(input: &str) -> impl Display {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.is_valid_2()).count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::util;

    use super::*;

    const YEAR: u32 = 2020;
    const DAY: u32 = 4;

    const EXAMPLE: &str = indoc::indoc! {"
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "2");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "233");
    }

    const INVALID_PASSPORTS: &str = indoc! {"
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007
    "};

    const VALID_PASSPORTS: &str = indoc! {"
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    "};

    #[test]
    fn part2_example() {
        assert_eq!(part2(INVALID_PASSPORTS).to_string(), "0");
        assert_eq!(part2(VALID_PASSPORTS).to_string(), "4");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "111");
    }
}

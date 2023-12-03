use crate::tools::vec_lines;
use regex::Regex;

pub fn convert(source: &str) -> u32 {
    match source {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => 0,
    }
}

pub fn extract_v2(source: String) -> u32 {
    let exp = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let exp_reverse = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();
    let caps = exp.captures_iter(source.as_str());
    let rev_source = source.chars().rev().collect::<String>();
    let caps_reverse = exp_reverse.captures_iter(rev_source.as_str());
    let coll_ord: Vec<u32> = caps
        .map(|c| {
            let (digit, _): (&str, [&str; 1]) = c.extract();
            convert(digit)
        })
        .collect();
    let coll_rev: Vec<u32> = caps_reverse
        .map(|c| {
            let (digit, _): (&str, [&str; 1]) = c.extract();
            convert(digit)
        })
        .collect();
    return coll_ord.first().unwrap() * 10 + coll_rev.first().unwrap();
}

pub fn extract_v1(source: String) -> u32 {
    let digits: Vec<u32> = source
        .chars()
        .filter_map(|c| {
            if c.is_digit(10) {
                Some(u32::from(c) - 48)
            } else {
                None
            }
        })
        .collect();

    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

pub fn sum_on_text(source: Vec<String>) -> u32 {
    source.iter().map(|s| extract_v2(s.into())).sum()
}

pub fn run() {
    println!("{:?}", sum_on_text(vec_lines("1_input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::e1::{extract_v1, extract_v2, sum_on_text};

    #[test]
    fn extract_simple() {
        assert_eq!(extract_v1("1abc2".to_string()), 12);
        assert_eq!(extract_v1("pqr3stu8vwx".to_string()), 38);
        assert_eq!(extract_v1("a1b2c3d4e5f".to_string()), 15);
        assert_eq!(extract_v1("treb7uchet".to_string()), 77);
    }
    #[test]
    fn extract_v2_simple() {
        assert_eq!(extract_v2("two1nine".to_string()), 29);
        assert_eq!(extract_v2("eightwothree".to_string()), 83);
        assert_eq!(extract_v2("abcone2threexyz".to_string()), 13);
        assert_eq!(extract_v2("xtwone3four".to_string()), 24);
        assert_eq!(extract_v2("4nineeightseven2".to_string()), 42);
        assert_eq!(extract_v2("zoneight234".to_string()), 14);
        assert_eq!(extract_v2("7pqrstsixteen".to_string()), 76);
        assert_eq!(
            extract_v2("6czklmzsmxgmktzxmxsixmnlfxonetwonesgj".to_string()),
            61
        );
    }

    #[test]
    fn extract_sum() {
        assert_eq!(
            sum_on_text(vec![
                "1abc2".to_string(),
                "pqr3stu8vwx".to_string(),
                "a1b2c3d4e5f".to_string(),
                "treb7uchet".to_string()
            ]),
            142
        );
        assert_eq!(
            sum_on_text(vec![
                "two1nine".to_string(),
                "eightwothree".to_string(),
                "abcone2threexyz".to_string(),
                "xtwone3four".to_string(),
                "4nineeightseven2".to_string(),
                "zoneight234".to_string(),
                "7pqrstsixteen".to_string(),
            ]),
            281
        );
    }
}

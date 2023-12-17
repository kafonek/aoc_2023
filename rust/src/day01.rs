use regex::Regex;
use std::collections::HashMap;

// lazy_static is kind of like lru_cache in that it'll only run this code once
lazy_static::lazy_static! {
    static ref MAPS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("one", "1");
        m.insert("two", "2");
        m.insert("three", "3");
        m.insert("four", "4");
        m.insert("five", "5");
        m.insert("six", "6");
        m.insert("seven", "7");
        m.insert("eight", "8");
        m.insert("nine", "9");
        m
    };
    static ref NUMBERS_PATTERN: Regex = {
        let pattern = MAPS.keys().cloned().collect::<Vec<_>>().join("|") + r"|\d";
        Regex::new(&pattern).unwrap()
    };
}

pub struct Calibration {
    raw: String,
}

impl Calibration {
    pub fn from_line(line: &str) -> Calibration {
        Calibration {
            raw: line.to_string(),
        }
    }

    pub fn value(&self) -> i32 {
        let digits: Vec<char> = self.raw.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let combined = format!("{}{}", first, last);
        combined.parse::<i32>().unwrap()
    }

    pub fn value2(&self) -> i32 {
        let mut digits = Vec::new();
        let mut start = 0;
        while let Some(matched) = NUMBERS_PATTERN.find(&self.raw[start..]) {
            let word_or_digit = matched.as_str();
            start += matched.start() + 1;

            let map_value = match MAPS.get(word_or_digit) {
                Some(v) => v,
                None => word_or_digit,
            };
            digits.push(map_value);
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let combined = format!("{}{}", first, last);
        combined.parse::<i32>().unwrap()
    }
}

use aoc_2023::day01::Calibration;

pub fn part1(input: String) -> String {
    let lines = input.lines();

    let mut answer = 0;
    for line in lines {
        let calibration = Calibration::from_line(&line);
        answer += calibration.value();
    }
    answer.to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines();

    let mut answer = 0;
    for line in lines {
        let calibration = Calibration::from_line(&line);
        answer += calibration.value2();
    }
    answer.to_string()
}

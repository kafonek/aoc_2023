use aoc_2023::day06::Race;
use web_sys::console;

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    // Python equivalent: times = lines[0].split(":")[1].split()
    let times: Vec<usize> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distances: Vec<usize> = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut results: Vec<usize> = Vec::new();
    // zip times and distances together
    for (time, distance) in times.iter().zip(distances.iter()) {
        let r = Race::new(*time, *distance);
        let result = r.run();
        results.push(result);
    }
    let answer = results.iter().product::<usize>();
    answer.to_string()
}

// TODO: Prod puzzle ends up with distance being larger than usize::MAX.
// Need to figure out how to handle that in wasm
pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    console::log_1(&format!("Max usize: {:?}", usize::MAX).into());
    console::log_1(&format!("lines: {:?}", lines).into());

    // Python equivalent: int("".join(lines[0].split(":")[1].split()))
    let time = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("Error parsing time into usize");

    console::log_1(&format!("time: {:?}", time).into());

    let distance = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("Error parsing distance into usize");

    console::log_1(&format!("distance: {:?}", distance).into());

    let race = Race::new(time, distance);
    let answer = race.run();
    answer.to_string()
}

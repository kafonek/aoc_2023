use aoc_2023::day07::Hand;

pub fn part1(input: String) -> String {
    let lines = input.lines();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        if let Some(hand) = Hand::from_string(line.to_string()) {
            hands.push(hand);
        }
    }
    hands.sort();

    let mut answer = 0;
    for (idx, hand) in hands.iter().enumerate() {
        let power = idx + 1;
        answer += hand.bid * power;
    }
    answer.to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        if let Some(hand) = Hand::from_string(line.replace('J', "X").to_string()) {
            hands.push(hand);
        }
    }
    hands.sort();

    let mut answer = 0;
    for (idx, hand) in hands.iter().enumerate() {
        let power = idx + 1;
        answer += hand.bid * power;
    }
    answer.to_string()
}

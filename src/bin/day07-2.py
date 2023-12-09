from pathlib import Path

from aoc_2023.day07 import Hand
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().replace('J', 'X').splitlines()
    hands = [Hand.from_string(line) for line in lines]
    answer = 0
    for idx, hand in enumerate(sorted(hands)):
        power = idx + 1
        value = hand.bid * power
        answer += value
    return str(answer)
    


if __name__ == "__main__":
    fp = Path("data/day07.txt")
    run_and_time(solve, fp)

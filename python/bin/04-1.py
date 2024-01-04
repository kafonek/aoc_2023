from pathlib import Path

from aoc_2023.day04 import Card
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()

    cards = [Card.from_string(line) for line in lines]

    answer = 0
    for card in cards:
        answer += card.score()

    return str(answer)


if __name__ == "__main__":
    fp = Path("../data/day04.txt")
    run_and_time(solve, fp)

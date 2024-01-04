import collections
from pathlib import Path

from aoc_2023.utils import run_and_time
from aoc_2023_pyo3.day04 import Card


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()

    cards = [Card.from_string(line) for line in lines]
    counts = collections.defaultdict(int)

    for card in cards:
        counts[card.id] += 1

    for card in cards:
        mult = counts[card.id]
        for copy in card.copies():
            counts[copy] += mult

    answer = sum(counts.values())

    return str(answer)


if __name__ == "__main__":
    fp = Path("../data/day04.txt")
    run_and_time(solve, fp)

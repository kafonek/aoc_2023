import collections
from pathlib import Path
from typing import List

from gridthings import Grid

from aoc_2023.day03 import Number, extract_numbers
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    text = fp.read_text()
    grid = Grid(text)

    nums: List[Number] = extract_numbers(grid)
    # "reverse" analyze this by building up dict of {Cell: [Number, Number]}
    # then we can count the number of cells that have two numbers and calculate
    # the product of those Number.value's
    gear_counts = collections.defaultdict(list)

    for n in nums:
        for gear in n.gears():
            gear_counts[gear].append(n)

    answer = 0
    for c, matches in gear_counts.items():
        if len(matches) == 2:
            gear_ratio = matches[0].value * matches[1].value
            answer += gear_ratio
    return str(answer)

if __name__ == '__main__':
    fp = Path("data/day03.txt")
    run_and_time(solve, fp)
from pathlib import Path
from typing import List

from gridthings import Cell, Grid

from aoc_2023.day03 import Number, extract_numbers
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    text = fp.read_text()
    grid = Grid(text)

    nums: List[Number] = extract_numbers(grid)
    answer = sum(n.value for n in nums if n.symbol_adjacent())
    return str(answer)

if __name__ == '__main__':
    fp = Path("data/day03.txt")
    run_and_time(solve, fp) 
import itertools
from pathlib import Path

from aoc_2023.day08 import Point
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    pattern = list(lines[0])

    points = {}
    for line in lines[2:]:
        p = Point.from_string(line)
        points[p.name] = p

    cycle = itertools.cycle(pattern)

    current = points["AAA"]
    steps = 0
    while True:
        if current.name == "ZZZ":
            break
        direction = next(cycle)
        if direction == "R":
            current = points[current.right]
        else:
            current = points[current.left]
        steps += 1
    return str(steps)


if __name__ == "__main__":
    fp = Path("data/day08.txt")
    run_and_time(solve, fp)

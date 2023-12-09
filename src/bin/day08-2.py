import itertools
from pathlib import Path

from aoc_2023.day08 import Point, Traverse
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    pattern = list(lines[0])

    points = {}
    for line in lines[2:]:
        p = Point.from_string(line)
        points[p.name] = p

    traverses = []
    for name, point in points.items():
        if name.endswith("A"):
            traverses.append(Traverse(current=point))

    print(len(traverses))

    cycle = itertools.cycle(pattern)

    steps = 0
    while True:
        direction = next(cycle)
        complete = True
        for traverse in traverses:
            traverse.step(direction, points)
            if not traverse.current.name.endswith("Z"):
                complete = False
        steps += 1
        if steps % 100 == 0:
            print(steps)
        if complete:
            break

    return str(traverses[0].steps)


if __name__ == "__main__":
    fp = Path("data/day08.txt")
    run_and_time(solve, fp)

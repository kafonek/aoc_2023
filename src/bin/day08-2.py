import itertools
from pathlib import Path
from typing import Dict, List

from aoc_2023.day08 import Point, PointIndex, Traverse
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    pattern = list(lines[0])

    point_list: List[Point] = []
    point_dict: Dict[str, Point] = {}
    for i, line in enumerate(lines[2:]):
        p = Point.from_string(line)
        p.index = i
        point_list.append(p)
        point_dict[p.name] = p

    current_indices = []
    end_point_indices = set()
    point_index = []

    for p in point_list:
        left = point_dict.get(p.left)
        right = point_dict.get(p.right)
        pi = PointIndex(idx=p.index, left_idx=left.index, right_idx=right.index)
        point_index.append(pi)
        if p.name.endswith('A'):
            current_indices.append(p.index)
        if p.name.endswith('Z'):
            end_point_indices.add(p.index)

    cycle = itertools.cycle(pattern)

    steps = 0
    while True:
        direction = next(cycle)
        complete = True
        for idx, value in enumerate(current_indices):
            pi = point_index[value]
            if direction == 'L':
                new = pi.left_idx
            else:
                new = pi.right_idx
            current_indices[idx] = new
            if new not in end_point_indices:
                complete = False
        steps += 1
        if steps % 10_000 == 0:
            print(steps)
        if complete:
            break
    return str(steps)


if __name__ == "__main__":
    fp = Path("data/day08.txt")
    run_and_time(solve, fp)

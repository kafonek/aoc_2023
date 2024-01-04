from pathlib import Path
from typing import List

from aoc_2023.day03 import Number
from aoc_2023.gridthings import Grid, Node
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    text = fp.read_text().strip()

    grid = Grid.from_string(text.strip())

    numbers: List[Number] = []
    current: List[Node] = []
    for row in grid.rows():
        for node in row:
            # if Node is a number, add it to the current set of nodes we're tracking
            if node.value.isdigit():
                current.append(node)

            # if it's not a number, check if we were tracking a set of numbers and save it off
            else:
                if current:
                    numbers.append(Number(nodes=current))
                    current = []
        # at the end of a row, if we were tracking a number then save it off
        if current:
            numbers.append(Number(nodes=current))
            current = []

    answer = 0
    for n in numbers:
        for neighbor in n.neighbors():
            if neighbor.value != "." and not neighbor.value.isdigit():
                answer += n.value
                break

    return str(answer)


if __name__ == "__main__":
    fp = Path("../data/day03.txt")
    run_and_time(solve, fp)

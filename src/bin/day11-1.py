import itertools
from pathlib import Path

import gridthings

from aoc_2023.utils import run_and_time


def manhattan_distance(cell1: gridthings.Cell, cell2: gridthings.Cell):
    return abs(cell2.y - cell1.y) + abs(cell2.x - cell1.x)

def solve(fp: Path) -> str:
    grid = gridthings.Grid(fp.read_text().strip())
    
    # expand out "empty" rows
    idx = 0
    while True:
        try:
            row = grid.get_row(idx)
        except KeyError:
            break
        if all(cell.value == '.' for cell in row):
            copy = [cell.copy(deep=True) for cell in row.cells]
            grid.insert_row(idx, copy)
            idx += 2
        else:
            idx += 1

    # expand out "empty" columns
    idx = 0
    while True:
        try:
            column = grid.get_column(idx)
        except KeyError:
            break
        if all(cell.value == '.' for cell in column):
            copy = [cell.copy(deep=True) for cell in column]
            grid.insert_column(idx, copy)
            idx += 2
        else:
            idx += 1

    galaxies = []
    for cell in grid.flatten():
        if cell.value == '#':
            galaxies.append(cell) 

    answer = 0
    for (cell1, cell2) in itertools.combinations(galaxies, 2):
        dist = manhattan_distance(cell1, cell2)
        answer += dist        

    return str(answer)


if __name__ == "__main__":
    fp = Path("data/day11.txt")
    run_and_time(solve, fp)

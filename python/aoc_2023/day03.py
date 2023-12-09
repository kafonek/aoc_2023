from dataclasses import dataclass
from typing import List

from gridthings import Cell, Grid, OutOfBoundsCell


@dataclass
class Number:
    cells: List[Cell]
    grid: Grid

    @property
    def value(self) -> int:
        s = "".join(c.value for c in self.cells)
        return int(s)

    def peek_all(self) -> List[Cell]:
        """
        Returns all adjacent cells to this Number, excluding cells belonging to this
        Number group or out of bounds cells
        """
        total_results = []
        for cell in self.cells:
            results = self.grid.peek_all(cell.y, cell.x)
            for item in results:
                if item in self.cells:
                    continue
                if isinstance(item, OutOfBoundsCell):
                    continue
                if item in total_results:
                    continue
                total_results.append(item)
        return total_results

    def symbol_adjacent(self) -> bool:
        for item in self.peek_all():
            if item.value != "." and not item.value.isdigit():
                return True
        return False

    def gears(self) -> List[Cell]:
        "Return the list of Gears (*) touching this Number"
        return [item for item in self.peek_all() if item.value == "*"]

    def __repr__(self):
        return f"<Number {self.value})>"


def extract_numbers(grid: Grid) -> List[Number]:
    collection: List[List[Cell]] = []
    current: List[Cell] = []
    # use grid.data.values() instead of grid.flatten() to avoid "wrap-around" numbers, e.g.
    # . . 3
    # 4 & .
    for row in grid.data.values():
        for c in row.values():
            if c.value.isdigit():
                current.append(c)
            else:
                if current:
                    collection.append(current)
                    current = []
        # end of row
        if current:
            collection.append(current)
            current = []

    nums: List[Number] = []
    for item in collection:
        nums.append(Number(cells=item, grid=grid))
    return nums

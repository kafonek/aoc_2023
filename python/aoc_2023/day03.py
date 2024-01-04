from dataclasses import dataclass
from typing import List

from aoc_2023.gridthings import Node


@dataclass
class Number:
    nodes: List[Node]

    @property
    def value(self) -> int:
        return int("".join([node.value for node in self.nodes]))

    def neighbors(self) -> List[Node]:
        results = []
        for node in self.nodes:
            for neighbor in node.neighbors():
                if (
                    neighbor is not None
                    and neighbor not in self.nodes
                    and neighbor not in results
                ):
                    results.append(neighbor)
        return results

    def __repr__(self):
        return f"<Number {self.value}>"

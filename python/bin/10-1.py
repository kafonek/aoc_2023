from pathlib import Path
from typing import List

from aoc_2023.gridthings import Grid, Node
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    text = fp.read_text().strip()

    grid = Grid.from_string(text.strip())

    # Create edges
    for node in grid.flatten():
        if node.value == "-":
            grid.add_edge(node, node.left())
            grid.add_edge(node, node.right())
        elif node.value == "|":
            grid.add_edge(node, node.up())
            grid.add_edge(node, node.down())
        elif node.value == "7":
            grid.add_edge(node, node.left())
            grid.add_edge(node, node.down())
        elif node.value == "J":
            grid.add_edge(node, node.up())
            grid.add_edge(node, node.left())
        elif node.value == "L":
            grid.add_edge(node, node.right())
            grid.add_edge(node, node.up())
        elif node.value == "F":
            grid.add_edge(node, node.down())
            grid.add_edge(node, node.right())

    # get start node and add edges
    for node in grid.flatten():
        if node.value == "S":
            for neighbor in node.linear():
                if neighbor and node in grid.get_edges(neighbor):
                    grid.add_edge(node, neighbor)
            break

    # pick a start path
    nxt_node, _weight = grid.get_edges(node).popitem()
    path: List[Node] = [node, nxt_node]
    while True:
        last_node = path[-2]
        current_node = path[-1]
        edges = list(grid.get_edges(current_node))
        nxt = edges[0] if edges[0] != last_node else edges[1]
        if nxt.value == "S":
            break
        path.append(nxt)

    answer = int(len(path) / 2)
    return str(answer)


if __name__ == "__main__":
    fp = Path("../data/day10.txt")
    run_and_time(solve, fp)

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

    # Figure out what the path[0] value is supposed to be based on path[-1] and path[1].
    # i.e. if they're the same row, then path[0] must be a horizontal connector ('-')
    if path[1].row == path[-1].row:  # same row, start connects horizontally
        path[0].value = "-"
    elif path[1].col == path[-1].col:  # same col, start connects vertically
        path[0].value = "|"
    elif path[1].row > path[-1].row:  # start path needs to route downwards
        if path[1].col > path[-1].col:  # route is coming from the left
            path[0].value = "7"
        else:
            path[0].value = "F"
    else:  # start path needs to route upwards
        if path[1].col > path[-1].col:  # path is coming from the left
            path[0].value = "J"
        else:
            path[0].value = "L"

    in_grid = []
    for node in grid.flatten():
        if node in path:
            continue
        wall_count = 0
        last_turn = None
        ray: List[Node] = [
            n for n in grid.get_row(node.row) if n in path and n.col > node.col
        ]
        for n in ray:
            if n.value == "-":  # don't count horizontal as part of the polygon
                continue
            if n.value == "|":
                wall_count += 1
            if n.value == "F":
                wall_count += 1
                last_turn = "F"
            if n.value == "L":
                wall_count += 1
                last_turn = "L"
            if n.value == "7":
                if last_turn == "F":
                    wall_count += 1
                last_turn = None
            if n.value == "J":
                if last_turn == "L":
                    wall_count += 1
                last_turn = None
        if wall_count % 2 == 1:
            in_grid.append(node)
    return str(len(in_grid))


if __name__ == "__main__":
    fp = Path("../data/day10.txt")
    run_and_time(solve, fp)

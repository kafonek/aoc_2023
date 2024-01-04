from typing import Any, Dict, List, Optional


class Node:
    def __init__(self, grid: "Grid", col: int, row: int, value: Any):
        self.grid = grid
        self.col = col
        self.row = row
        self.value = value

    def __repr__(self) -> str:
        return f"<Node ({self.col}, {self.row}) = {self.value}>"

    def __hash__(self) -> int:
        return hash((self.col, self.row))

    # Neighbors
    def left(self, step: int = 1) -> Optional["Node"]:
        return self.grid.left(self.col, self.row, step)

    def right(self, step: int = 1) -> Optional["Node"]:
        return self.grid.right(self.col, self.row, step)

    def up(self, step: int = 1) -> Optional["Node"]:
        return self.grid.up(self.col, self.row, step)

    def down(self, step: int = 1) -> Optional["Node"]:
        return self.grid.down(self.col, self.row, step)

    def horizontal(self, step: int = 1) -> List[Optional["Node"]]:
        return self.grid.horizontal(self.col, self.row, step)

    def vertical(self, step: int = 1) -> List[Optional["Node"]]:
        return self.grid.vertical(self.col, self.row, step)

    def linear(self, step: int = 1) -> List[Optional["Node"]]:
        return self.grid.linear(self.col, self.row, step)

    def up_left(self, step: int = 1) -> Optional["Node"]:
        return self.grid.up_left(self.col, self.row, step)

    def up_right(self, step: int = 1) -> Optional["Node"]:
        return self.grid.up_right(self.col, self.row, step)

    def down_left(self, step: int = 1) -> Optional["Node"]:
        return self.grid.down_left(self.col, self.row, step)

    def down_right(self, step: int = 1) -> Optional["Node"]:
        return self.grid.down_right(self.col, self.row, step)

    def diagonal(self, step: int = 1) -> List[Optional["Node"]]:
        return self.grid.diagonal(self.col, self.row, step)

    def neighbors(self, step: int = 1) -> List[Optional["Node"]]:
        return self.grid.neighbors(self.col, self.row, step)


class Grid:
    def __init__(self):
        self.nodes: List[List[Node]] = []
        self.edges: Dict[Node, Dict[Node, int]] = {}

    def add_row(self, row: List[Node]) -> None:
        self.nodes.append(row)

    def add_edge(self, node1: Node, node2: Node, weight: int = 1) -> None:
        if node1 not in self.edges:
            self.edges[node1] = {}
        self.edges[node1][node2] = weight

    def get_edges(self, node: Node) -> Dict[Node, int]:
        return self.edges.get(node, {})

    @classmethod
    def from_string(cls, string: str) -> "Grid":
        grid = cls()
        for row_num, characters in enumerate(string.splitlines()):
            nodes = []
            for col_num, character in enumerate(characters):
                node = Node(grid, col_num, row_num, character)
                nodes.append(node)
            grid.add_row(nodes)
        return grid

    @property
    def shape(self) -> (int, int):
        return len(self.nodes), len(self.nodes[0])

    def __repr__(self) -> str:
        return f"<Grid {self.shape}>"

    def get_node(self, col: int, row: int) -> Optional[Node]:
        if 0 <= row < len(self.nodes) and 0 <= col < len(self.nodes[row]):
            return self.nodes[row][col]

    def get_row(self, row: int) -> List[Node]:
        return self.nodes[row]

    def get_col(self, col: int) -> List[Node]:
        return [row[col] for row in self.nodes]

    def rows(self) -> List[List[Node]]:
        return self.nodes

    def cols(self) -> List[List[Node]]:
        return [self.get_col(col) for col in range(self.shape[1])]

    def flatten(self) -> List[Node]:
        return [node for row in self.rows() for node in row]

    # Neighbors
    def left(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col - step, row)

    def right(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col + step, row)

    def up(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col, row - step)

    def down(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col, row + step)

    def horizontal(self, col: int, row: int, step: int = 1) -> List[Node]:
        results = []
        for node in [self.left(col, row, step), self.right(col, row, step)]:
            if node is not None:
                results.append(node)
        return results

    def vertical(self, col: int, row: int, step: int = 1) -> List[Node]:
        results = []
        for node in [self.up(col, row, step), self.down(col, row, step)]:
            if node is not None:
                results.append(node)
        return results

    def linear(self, col: int, row: int, step: int = 1) -> List[Node]:
        return self.horizontal(col, row, step) + self.vertical(col, row, step)

    def up_left(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col - step, row - step)

    def up_right(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col + step, row - step)

    def down_left(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col - step, row + step)

    def down_right(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        return self.get_node(col + step, row + step)

    def diagonal(self, col: int, row: int, step: int = 1) -> List[Node]:
        results = []
        for node in [
            self.up_left(col, row, step),
            self.up_right(col, row, step),
            self.down_left(col, row, step),
            self.down_right(col, row, step),
        ]:
            if node is not None:
                results.append(node)
        return results

    def neighbors(self, col: int, row: int, step: int = 1) -> List[Optional[Node]]:
        return self.linear(col, row, step) + self.diagonal(col, row, step)

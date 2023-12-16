from typing import Any, Dict, Iterator, List, Optional, Tuple


class Node:
    def __init__(self, graph: 'Graph', col: int, row: int, value: Any, extra: Optional[dict] = None):
        self.graph = graph
        self.col = col
        self.row = row
        self.value = value
        self.extra = extra or {}      

    def __repr__(self):
        return f"<Node ({self.col} {self.row}): {self.value}>"
    
    def __hash__(self):
        return hash((self.col, self.row)) 
    
    def right(self, step: int = 1) -> Optional['Node']:
        return self.graph.neighbor_right(self.col, self.row, step)
    
    def left(self, step: int = 1) -> Optional['Node']:
        return self.graph.neighbor_left(self.col, self.row, step)
    
    def up(self, step: int = 1) -> Optional['Node']:
        return self.graph.neighbor_up(self.col, self.row, step)
    
    def down(self, step: int = 1) -> Optional['Node']:
        return self.graph.neighbor_down(self.col, self.row, step)
    
    def neighbors_horizontal(self, step: int = 1) -> List['Node']:
        return self.graph.neighbors_horizontal(self.col, self.row, step)
    
    def neighbors_vertical(self, step: int = 1) -> List['Node']:
        return self.graph.neighbors_vertical(self.col, self.row, step)
    
    def neighbors(self, step: int = 1) -> List['Node']:
        return self.graph.neighbors(self.col, self.row, step)
    
    def add_edge(self, node: 'Node', weight: int = 1, extra: Optional[dict] = None):
        self.graph.add_edge(self, node, weight, extra)

    def swap(self, node: 'Node'):
        self.graph.swap(self, node)


    def find_all_paths(self) -> List[List['Node']]:
        return self.graph.find_all_paths(self)

class Edge:
    def __init__(self, node1: Node, node2: Node, weight: int = 1, extra: Optional[dict] = None):
        self.node1 = node1
        self.node2 = node2
        self.weight = weight
        self.extra = extra or {}

    def __repr__(self):
        return f"<Edge {self.node1.value} -> {self.node2.value}>"
        

class Graph:
    def __init__(self):
        self.nodes: List[List[Node]] = []
        self.edges: Dict[Node, Dict[Node, Edge]] = {}
        
    @property
    def shape(self) -> Tuple[int, int]:
        "Return (row_count, col_count)"
        return (len(self.nodes), len(self.nodes[0]))
    
    def add_row(self, row: List[Node]):
        self.nodes.append(row)

    def add_edge(self, node1: Node, node2: Node, weight=1, extra: Optional[dict] = None):
        if node1 not in self.edges:
            self.edges[node1] = {}
        self.edges[node1][node2] = Edge(node1, node2, weight, extra)

    @staticmethod
    def from_string(input_string: str) -> 'Graph':
        lines = input_string.split('\n')
        graph = Graph()
        for row_idx, line in enumerate(lines):
            row = []
            for col_idx, char in enumerate(line):
                row.append(Node(graph, col_idx, row_idx, char))
            graph.add_row(row)
        return graph

    def get_node(self, col: int, row: int) -> Optional[Node]:
        try:
            return self.nodes[row][col]
        except IndexError:
            return None
        
    def get_row(self, row: int) -> List[Node]:
        return self.nodes[row]

    def get_col(self, col: int) -> List[Node]:
        return [row[col] for row in self.nodes]

    def rows(self) -> Iterator[List[Node]]:
        return iter(self.nodes)

    def cols(self) -> Iterator[List[Node]]:
        for idx in range(len(self.nodes[0])):
            yield self.get_col(idx)

    def flatten(self) -> Iterator[Node]:
        for row in self.rows():
            for node in row:
                yield node
    
    def swap(self, node1: Node, node2: Node) -> None:
        node1.col, node2.col = node2.col, node1.col
        node1.row, node2.row = node2.row, node1.row
        self.nodes[node1.col][node1.row], self.nodes[node2.col][node2.row] = node2, node1
    
    def neighbor_right(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        "Return the node to the right of the given node, if it exists"
        return self.get_node(col+step, row)

    def neighbor_left(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        "Return the node to the left of the given node, if it exists"
        return self.get_node(col-step, row)
    
    def neighbor_up(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        "Return the node above the given node, if it exists"
        return self.get_node(col, row-step)
    
    def neighbor_down(self, col: int, row: int, step: int = 1) -> Optional[Node]:
        "Return the node below the given node, if it exists"
        return self.get_node(col, row+step)
    
    def neighbors_horizontal(self, col: int, row: int, step: int = 1) -> List[Node]:
        "Return the nodes to the left and right of the given node, if they exist"
        return [self.neighbor_left(col, row, step), self.neighbor_right(col, row, step)]
    
    def neighbors_vertical(self, col: int, row: int, step: int = 1) -> List[Node]:
        "Return the nodes above and below the given node, if they exist"
        return [self.neighbor_up(col, row, step), self.neighbor_down(col, row, step)]
    
    def neighbors(self, col: int, row: int, step: int = 1) -> List[Node]:
        "Return the nodes to the left, right, above, and below the given node, if they exist"
        return self.neighbors_horizontal(col, row, step) + self.neighbors_vertical(col, row, step)
    
    def line_right(self, col: int, row: int) -> List[Node]:
        "Return a list of nodes to the right of the given node, if they exist"
        return self.nodes[row][col+1:]
    
    def line_left(self, col: int, row: int) -> List[Node]:
        "Return a list of nodes to the left of the given node, if they exist"
        return self.nodes[row][:col][::-1]
    
    def line_up(self, col: int, row: int) -> List[Node]:
        "Return a list of nodes above the given node, if they exist"
        return [row[col] for row in self.nodes[:row]][::-1]
    
    def line_down(self, col: int, row: int) -> List[Node]:
        "Return a list of nodes below the given node, if they exist"
        return [row[col] for row in self.nodes[row+1:]]

    def find_all_paths(self, start_node: Node) -> List[List[Node]]:
        all_paths = []
        stack = [(start_node, [start_node])]
        while stack:
            (node, path) = stack.pop()
            if node not in self.edges:
                continue
            for edge in self.edges[node]:
                neighbor = edge.node2
                if neighbor in path:
                    continue



        return all_paths
    
    def __str__(self):
        output = ""
        for row in self.rows():
            for node in row:
                output += node.value
            output += "\n"
        return output

    def __repr__(self):
        return f"<Graph {self.shape}>"
        

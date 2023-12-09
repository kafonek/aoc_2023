from typing import List

# src/py_bindings/day02.rs
class Bag:
    red: int
    green: int
    blue: int
    def __init__(self, red: int, green: int, blue: int): ...
    def from_string(s: str) -> Bag: ...
    def can_contain(self, other: Bag) -> bool: ...
    def power(self) -> int: ...

class Game:
    id: int
    bags: List[Bag]
    def __init__(self, id: int, bag: Bag): ...
    def from_string(s: str) -> Game: ...
    def check(self, max_bag: Bag) -> bool: ...
    def max_values(self) -> Bag: ...

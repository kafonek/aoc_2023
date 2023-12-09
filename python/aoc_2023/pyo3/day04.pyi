from typing import List

# src/py_bindings/day04.rs
class Card:
    id: int
    def from_string(s: str) -> Card: ...
    def score(self) -> int: ...
    def copies(self) -> List[int]: ...
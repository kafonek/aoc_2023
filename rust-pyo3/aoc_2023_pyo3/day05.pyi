from typing import List

class Pipeline:
    @staticmethod
    def from_lines(lines: List[str]) -> Pipeline: ...
    def get(self, seed: int, debug: bool) -> int: ...

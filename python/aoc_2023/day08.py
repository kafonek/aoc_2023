from dataclasses import dataclass
from typing import Dict, Literal


@dataclass
class Point:
    name: str
    left: str
    right: str

    @classmethod
    def from_string(cls, s: str):
        # sample: AAA = (BBB, CCC)
        left1, right1 = s.split("=")  # (AAA, (BBB, CCC))
        left2, right2 = right1.strip().strip("()").split(",")  # (BBB, CCC)
        return cls(name=left1.strip(), left=left2.strip(), right=right2.strip())


@dataclass
class Traverse:
    current: Point

    def step(self, direction: Literal["R", "L"], points: Dict[str, Point]):
        if direction == "R":
            self.current = points[self.current.right]
        else:
            self.current = points[self.current.left]

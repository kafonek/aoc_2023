from dataclasses import dataclass
from typing import List


def make_diffs(numbers: List[int]):
    return [j - i for i, j in zip(numbers, numbers[1:])]


def make_diff_hierarchy(numbers: List[int]):
    diffs = make_diffs(numbers)
    total_diffs = [diffs]
    while len(diffs) > 1:
        diffs = make_diffs(diffs)
        total_diffs.append(diffs)
    return total_diffs


@dataclass
class Pattern:
    numbers: List[int]

    @staticmethod
    def from_line(s: str):
        return Pattern([int(c) for c in s.split()])

    def next_number(self):
        diff_hierarchy = make_diff_hierarchy(self.numbers)
        changes = sum([item[-1] for item in diff_hierarchy])
        return self.numbers[-1] + changes

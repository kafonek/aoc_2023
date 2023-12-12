from pathlib import Path

from aoc_2023.day09 import make_diff_hierarchy, previous_diffs
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().split("\n")
    answer = 0
    for line in lines:
        nums = [int(c) for c in line.split()]
        diffs = make_diff_hierarchy(nums)
        pd = previous_diffs(diffs)
        next_number = nums[0] - pd[-1]
        answer += next_number
    return str(answer)


if __name__ == "__main__":
    fp = Path("data/day09.txt")
    run_and_time(solve, fp)

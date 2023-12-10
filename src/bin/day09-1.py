from aoc_2023.day09 import Pattern
from aoc_2023.utils import run_and_time
from pathlib import Path


def solve(fp: Path) -> str:
    lines = fp.read_text().split("\n")
    patterns = [Pattern.from_line(line) for line in lines]
    answer = 0
    for pattern in patterns:
        answer += pattern.next_number()
    return str(answer)


if __name__ == "__main__":
    fp = Path("data/day09.txt")
    run_and_time(solve, fp)

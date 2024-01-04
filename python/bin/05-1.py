import math
from pathlib import Path

from aoc_2023.day05 import Pipeline
from aoc_2023.utils import run_and_time


def solve(fp: Path):
    lines = fp.read_text().splitlines()
    seeds = lines[0].split(":")[1].split()
    pipeline = Pipeline.from_lines(lines[2:])
    answer = math.inf
    for seed in seeds:
        result = pipeline.get(int(seed))
        if result < answer:
            answer = result
    return str(answer)


if __name__ == "__main__":
    fp = Path("../data/day05.txt")
    run_and_time(solve, fp)

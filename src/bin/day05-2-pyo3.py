import concurrent.futures
from pathlib import Path
from typing import List

from aoc_2023.pyo3.day05 import Pipeline
from aoc_2023.utils import run_and_time


def solve(fp: Path):
    lines = fp.read_text().splitlines()
    ranges: List[range] = []

    seed_numbers = lines[0].split(':')[1].split()
    while seed_numbers:
        start = int(seed_numbers.pop(0))
        end = start + int(seed_numbers.pop(0))
        ranges.append(range(start, end))

    pipeline = Pipeline.from_lines(lines[2:])
    
    n_generator = (i for r in ranges for i in r)

    # here because the Pipeline.get rust code takes debug arg and can't be default-valued like
    # regular Python methods
    def wrapper(n: int):
        return pipeline.get(n, debug=False)

    with concurrent.futures.ProcessPoolExecutor() as executor:
        results = executor.map(wrapper, n_generator)

    answer = min(results)
    return str(answer)

if __name__ == '__main__':
    fp = Path("data/day05.txt")
    run_and_time(solve, fp)
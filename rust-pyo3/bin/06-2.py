from pathlib import Path

from aoc_2023.utils import run_and_time
from aoc_2023_pyo3.day06 import Race


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    t = int("".join(lines[0].split(":")[1].split()))
    d = int("".join(lines[1].split(":")[1].split()))

    r = Race(time=t, distance_to_beat=d)
    return str(r.run())


if __name__ == "__main__":
    fp = Path("../data/day06.txt")
    run_and_time(solve, fp)

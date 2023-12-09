import math
from pathlib import Path

from aoc_2023.pyo3.day06 import Race
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    times = lines[0].split(':')[1].split()
    distances = lines[1].split(':')[1].split()

    races = []
    for (t, d) in zip(times, distances):
        r = Race(duration=int(t), distance_to_beat=int(d))
        r.run()
        races.append(r)
        
    answer = math.prod(r.answer for r in races)
    return str(answer)

if __name__ == '__main__':
    fp = Path("data/day06.txt")
    run_and_time(solve, fp)
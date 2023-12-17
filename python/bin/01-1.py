from pathlib import Path

from aoc_2023.day01 import Calibration
from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    calibrations = [Calibration.from_line(line) for line in lines]
    return str(sum(calibration.value() for calibration in calibrations))

if __name__ == "__main__":
    fp = Path("../data/day01.txt")
    run_and_time(solve, fp)
from pathlib import Path

from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()
    extracted_digits = []
    for line in lines:
        nums = [c for c in line if c.isdigit()]
        extracted_digits.append(nums)
        
    combined = []
    for item in extracted_digits:
        s = item[0] + item[-1]
        combined.append(int(s))
        
    answer = sum(combined)   
    return str(answer)

if __name__ == "__main__":
    fp = Path("data/day01.txt")
    run_and_time(solve, fp)
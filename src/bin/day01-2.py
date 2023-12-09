import re
from pathlib import Path

from aoc_2023.utils import run_and_time


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()

    maps = {'one': '1',
            'two' : '2',
            'three': '3',
            'four': '4',
            'five': '5',
            'six': '6',
            'seven': '7',
            'eight': '8',
            'nine': '9'}

    combined_pattern = '|'.join(maps.keys()) + '|\d'
    pattern = re.compile(combined_pattern)
    extracted_digits = []

    for line in lines:
        matches = []
        for i in range(len(line)):
            match = pattern.match(line, i)
            if match:
                word_or_digit = match.group()
                if word_or_digit in maps:
                    matches.append(maps[word_or_digit])
                else:
                    matches.append(word_or_digit)
        extracted_digits.append(matches)
        
    combined = []
    for item in extracted_digits:
        s = item[0] + item[-1]
        combined.append(int(s))  
    
    answer = sum(combined)
    return str(answer)

if __name__ == "__main__":
    fp = Path("data/day01.txt")
    run_and_time(solve, fp)
from pathlib import Path
from typing import List

from aoc_2023.utils import run_and_time
from aoc_2023_pyo3.day02 import Game


def solve(fp: Path) -> str:
    lines = fp.read_text().splitlines()

    games: List[Game] = []
    for line in lines:
        game = Game.from_string(line)
        games.append(game)

    answer = 0
    for game in games:
        answer += game.max_values().power()

    return str(answer)    

if __name__ == "__main__":
    fp = Path("../data/day02.txt")
    run_and_time(solve, fp)            
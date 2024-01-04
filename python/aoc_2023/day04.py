from dataclasses import dataclass
from typing import List


@dataclass
class Card:
    id: int
    winning_numbers: List[int]
    guess_numbers: List[int]

    @classmethod
    def from_string(cls, s: str) -> "Card":
        card, numbers = s.split(":", 1)
        id = int(card.split()[1])
        (left, right) = numbers.split("|", 1)
        winning_numbers = left.split()
        guess_numbers = right.split()
        return cls(id=id, winning_numbers=winning_numbers, guess_numbers=guess_numbers)

    def score(self) -> int:
        score = 0
        for n in self.guess_numbers:
            if n in self.winning_numbers:
                if not score:
                    score = 1
                else:
                    score *= 2
        return score

    def copies(self) -> List[int]:
        copy_range = len([n for n in self.guess_numbers if n in self.winning_numbers])
        start = self.id + 1
        stop = start + copy_range
        return list(range(start, stop))

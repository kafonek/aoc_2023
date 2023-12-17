import re
from dataclasses import dataclass
from functools import lru_cache

maps = {'one': '1',
        'two' : '2',
        'three': '3',
        'four': '4',
        'five': '5',
        'six': '6',
        'seven': '7',
        'eight': '8',
        'nine': '9'}

@lru_cache
def numbers_pattern():
    combined_pattern = '|'.join(maps.keys()) + r"|\d"
    pattern = re.compile(combined_pattern)
    return pattern

@dataclass
class Calibration:
    raw: str

    @staticmethod
    def from_line(line: str):
        return Calibration(line)
    
    def value(self) -> int:
        "Return the part-1 digit extraction (characters that are digits only)"
        digits = [c for c in self.raw if c.isdigit()]
        return int(digits[0] + digits[-1])
    
    def value2(self) -> int:
        "Return the part-2 digit extraction (spelled out or actual digits)"
        pattern = numbers_pattern()
        # Need to do this loop because some numbers have overlapping characters (eightwo -> 8,2)
        matches = []
        for i in range(len(self.raw)):
            match = pattern.match(self.raw, i)
            if match:
                # match could be a digit or a word. If it's a word, convert to digit with map lookup
                word_or_digit = match.group()
                if word_or_digit in maps:
                    matches.append(maps[word_or_digit])
                else:
                    matches.append(word_or_digit)
        return int(matches[0] + matches[-1])
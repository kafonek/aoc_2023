
from dataclasses import dataclass
from typing import List, Optional


@dataclass
class Bag:
    blue: Optional[int] = 0
    red: Optional[int] = 0
    green: Optional[int] = 0

    @classmethod
    def from_string(cls, s: str):
        "input should look something like: 3 blue, 4 red, 2 green"
        d = {}
        for part in s.strip().split(','):
            count, color = part.split()
            d[color] = int(count)
        return cls(**d)

    def can_contain(self, other: "Bag"):
        return self.red >= other.red and self.green >= other.green and self.blue >= other.blue
    
    def power(self) -> int: 
        return self.red * self.green * self.blue
    

@dataclass
class Game:
    id: int
    bags: List[Bag]

    @classmethod
    def from_string(cls, s: str):
        "input sould look something like: Game 1: 3 blue; ..."
        id_part, bag_parts = s.split(':', 1)
        id = id_part.split()[-1]
        bags = []
        for part in bag_parts.split(';'):
            bag = Bag.from_string(part)
            bags.append(bag)
        return cls(id=int(id), bags=bags)

    def check(self, max_bag: Bag):
        for bag in self.bags:
            if not max_bag.can_contain(bag):
                return False
        return True    
    
    def max_values(self) -> Bag:
        max_red = max_green = max_blue = 0
        for bag in self.bags:
            max_red = max(max_red, bag.red)
            max_green = max(max_green, bag.green)
            max_blue = max(max_blue, bag.blue)
        return Bag(red=max_red, green=max_green, blue=max_blue)
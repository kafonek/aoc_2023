from typing import List

class Card:
    id: int
    def score(self) -> int: ...
    def copies(self) -> List[int]: ...
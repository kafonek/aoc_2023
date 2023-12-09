import enum
from dataclasses import dataclass
from typing import List, Tuple


class Card(enum.Enum):
    "AKQJT98765432"
    TWO = 2
    THREE = 3
    FOUR = 4
    FIVE = 5
    SIX = 6
    SEVEN = 7
    EIGHT = 8
    NINE = 9
    TEN = 10
    JACK = 11
    QUEEN = 12
    KING = 13
    ACE = 14
    JOKER = 1

    @staticmethod
    def from_string(s: str):
        mapping = {
            "2": Card.TWO,
            "3": Card.THREE,
            "4": Card.FOUR,
            "5": Card.FIVE,
            "6": Card.SIX,
            "7": Card.SEVEN,
            "8": Card.EIGHT,
            "9": Card.NINE,
            "T": Card.TEN,
            "J": Card.JACK,
            "Q": Card.QUEEN,
            "K": Card.KING,
            "A": Card.ACE,
            "X": Card.JOKER,
        }
        return mapping[s]

    
class HandType(enum.Enum):
    HIGH_CARD = 1
    ONE_PAIR = 2
    TWO_PAIR = 3
    THREE_OF_A_KIND = 4
    FULL_HOUSE = 5
    FOUR_OF_A_KIND = 6
    FIVE_OF_A_KIND = 7
    
    @staticmethod
    def from_cards(cards: List[Card]):
        """
        Jokers are a special case here. They can be a match to any other card for the purpose of
        making the highest value HandType.
        """
        assert len(cards) == 5
        
        counts = {}
        for card in cards:
            if card not in counts:
                counts[card] = 0
            counts[card] += 1

        # For any jokers, add them to the count of the most common card
        if Card.JOKER in counts:
            most_common = None
            for card, count in counts.items():
                if most_common is None or count > counts[most_common]:
                    most_common = card
            counts[most_common] += counts[Card.JOKER]

        # Five of a kind
        if 5 in counts.values():
            return HandType.FIVE_OF_A_KIND
        
        # Four of a kind
        if 4 in counts.values():
            return HandType.FOUR_OF_A_KIND
        
        # Full house
        if 3 in counts.values() and 2 in counts.values():
            return HandType.FULL_HOUSE
        
        # Three of a kind
        if 3 in counts.values():
            return HandType.THREE_OF_A_KIND
        
        # Two pair
        if list(counts.values()).count(2) == 2:
            return HandType.TWO_PAIR
        
        # One pair
        if 2 in counts.values():
            return HandType.ONE_PAIR
        
        # High card
        return HandType.HIGH_CARD


        

       


        
        

@dataclass
class Hand:
    cards: List[Card]
    hand_type: HandType
    value: Tuple[int]
    bid: int
    
    @staticmethod
    def from_string(s: str):
        "Line looks like: 32T3K 765 (cards and bid)"
        card_symbols, bid = s.split()
        cards = [Card.from_string(c) for c in card_symbols]
        hand_type = HandType.from_cards(cards)
        value = (hand_type.value, *(c.value for c in cards))
        return Hand(cards, hand_type, value, int(bid))
    
    def __lt__(self, other: 'Hand'):
        return self.value < other.value
    
    
    

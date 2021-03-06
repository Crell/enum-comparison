import enum

class Suit(enum.Enum):
    HEARTS = enum.auto()
    DIAMONDS = enum.auto()
    CLUBS = 'C'
    SPADES = "S"

    def color(self):
        if self in [self.CLUBS, self.SPADES]:
            return "Black"
        else:
            return "Red"


for card in (Suit):
    print ("As a string: " + str(card))
    # print (card)
    print ("As a repr: ")
    print (repr(card))
    print (card.color())
    print('-----')

print ("Are they equal?")
print (Suit.HEARTS == Suit.HEARTS)

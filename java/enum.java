enum Card {
  HEARTS,
  DIAMONDS,
  CLUBS,
  SPADES;

  public String color() {
    switch (this) {
      case SPADES:
        return "Swords of a soldier";
      case CLUBS:
        return "Weapons of war";
      case DIAMONDS:
        return "Money for this art";
      default:
        return "Shape of my heart";
    }
  }
}

class Main {

  public static void main(String[] args) {
    for (Card c : Card.values()) {
      System.out.println(c);
    }

    System.out.println("Suit is: %s" + Card.valueOf("SPADES"));
    System.out.println("Suit is: %s" + Card.valueOf("SPADES").ordinal());
  }

}

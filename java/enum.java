enum Card {
HEARTS,
DIAMONDS,
CLUBS,
SPADES
}

class Main {

  public static void main(String[] args) {
    for (Card c : Card.values()) {
      System.out.println(c);
    }


    System.out.println("Suit is: %s", Card::SPADES);
  }

}

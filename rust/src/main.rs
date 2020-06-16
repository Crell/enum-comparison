#![allow(dead_code)]

use std::fmt::{self, Formatter, Display};


#[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
enum Card {
    Hearts(i8),
    Diamonds(i8),
    Clubs(i8),
    Spades(i8),
}

impl Suit {
    fn color(&self) -> String {
        match self {
            Self::Hearts => "Red".to_string(),
            Self::Diamonds => "Red".to_string(),
            Self::Clubs => "Black".to_string(),
            Self::Spades => "Black".to_string(),
        }
    }
}

impl Card {
    fn pair_with(self, other: Self) -> bool {
        use Card::*;
        let the_val = match self {
            Clubs(x) | Hearts(x) | Spades(x) | Diamonds(x) => x
        };

        let other_val = match other {
            Clubs(x) | Hearts(x) | Spades(x) | Diamonds(x) => x
        };
        the_val == other_val
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Hearts(x) => write!(f, "{} of Hearts", x),
            Self::Diamonds(x) => write!(f, "{} of Diamonds", x),
            Self::Clubs(x) => write!(f, "{} of Clubs", x),
            Self::Spades(x) => write!(f, "{} of Spades", x),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Hearts => write!(f, "Hearts"),
            Self::Diamonds => write!(f, "Diamonds"),
            Self::Clubs => write!(f, "Clubs"),
            Self::Spades => write!(f, "Spades"),
        }
    }
}

fn main() {
    println!("{}", Suit::Clubs);
    println!("{:?}", Suit::Hearts);
    println!("{}", Suit::Diamonds.color());

    println!("{}", Card::Spades(4));
}

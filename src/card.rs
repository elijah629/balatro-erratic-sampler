#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Value {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 11,
    King = 13,
    Ace = 15,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card(pub Value, pub Suit);

pub const CARDS: [Card; 52] = [
    // Clubs
    Card(Value::Two, Suit::Club),   // C_2
    Card(Value::Three, Suit::Club), // C_3
    Card(Value::Four, Suit::Club),  // C_4
    Card(Value::Five, Suit::Club),  // C_5
    Card(Value::Six, Suit::Club),   // C_6
    Card(Value::Seven, Suit::Club), // C_7
    Card(Value::Eight, Suit::Club), // C_8
    Card(Value::Nine, Suit::Club),  // C_9
    Card(Value::Ace, Suit::Club),   // C_A
    Card(Value::Jack, Suit::Club),  // C_J
    Card(Value::King, Suit::Club),  // C_K
    Card(Value::Queen, Suit::Club), // C_Q
    Card(Value::Ten, Suit::Club),   // C_T
    // Diamonds
    Card(Value::Two, Suit::Diamond),   // D_2
    Card(Value::Three, Suit::Diamond), // D_3
    Card(Value::Four, Suit::Diamond),  // D_4
    Card(Value::Five, Suit::Diamond),  // D_5
    Card(Value::Six, Suit::Diamond),   // D_6
    Card(Value::Seven, Suit::Diamond), // D_7
    Card(Value::Eight, Suit::Diamond), // D_8
    Card(Value::Nine, Suit::Diamond),  // D_9
    Card(Value::Ace, Suit::Diamond),   // D_A
    Card(Value::Jack, Suit::Diamond),  // D_J
    Card(Value::King, Suit::Diamond),  // D_K
    Card(Value::Queen, Suit::Diamond), // D_Q
    Card(Value::Ten, Suit::Diamond),   // D_T
    // Hearts
    Card(Value::Two, Suit::Heart),   // H_2
    Card(Value::Three, Suit::Heart), // H_3
    Card(Value::Four, Suit::Heart),  // H_4
    Card(Value::Five, Suit::Heart),  // H_5
    Card(Value::Six, Suit::Heart),   // H_6
    Card(Value::Seven, Suit::Heart), // H_7
    Card(Value::Eight, Suit::Heart), // H_8
    Card(Value::Nine, Suit::Heart),  // H_9
    Card(Value::Ace, Suit::Heart),   // H_A
    Card(Value::Jack, Suit::Heart),  // H_J
    Card(Value::King, Suit::Heart),  // H_K
    Card(Value::Queen, Suit::Heart), // H_Q
    Card(Value::Ten, Suit::Heart),   // H_T
    // Spades
    Card(Value::Two, Suit::Spade),   // S_2
    Card(Value::Three, Suit::Spade), // S_3
    Card(Value::Four, Suit::Spade),  // S_4
    Card(Value::Five, Suit::Spade),  // S_5
    Card(Value::Six, Suit::Spade),   // S_6
    Card(Value::Seven, Suit::Spade), // S_7
    Card(Value::Eight, Suit::Spade), // S_8
    Card(Value::Nine, Suit::Spade),  // S_9
    Card(Value::Ace, Suit::Spade),   // S_A
    Card(Value::Jack, Suit::Spade),  // S_J
    Card(Value::King, Suit::Spade),  // S_K
    Card(Value::Queen, Suit::Spade), // S_Q
    Card(Value::Ten, Suit::Spade),   // S_T
];

pub const CARDS_LAST_INDEX: usize = CARDS.len() - 1;

use std::fmt;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self.0 {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "T",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        };
        let s = match self.1 {
            Suit::Heart => "H",
            Suit::Club => "C",
            Suit::Diamond => "D",
            Suit::Spade => "S",
        };
        write!(f, "{}_{}", s, v)
    }
}

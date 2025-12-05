use std::fmt;

/// Represents the rank of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    /// Returns the base value of the rank (Ace=1, Face cards=10, others=face value)
    pub fn base_value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }

    /// Returns all possible ranks in order
    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self {
            Rank::Ace => "Ace",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(f, "{}", rank_str)
    }
}

/// Represents the suit of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    /// Returns all possible suits
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_str = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit_str)
    }
}

/// Represents a single playing card with a rank and suit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Creates a new card with the specified rank and suit
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    /// Returns the base value of the card (Ace=1, Face cards=10, others=face value)
    pub fn base_value(&self) -> u8 {
        self.rank.base_value()
    }

    /// Returns the rank of the card
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Returns the suit of the card
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Property-based test generators
    fn any_rank() -> impl Strategy<Value = Rank> {
        prop_oneof![
            Just(Rank::Ace),
            Just(Rank::Two),
            Just(Rank::Three),
            Just(Rank::Four),
            Just(Rank::Five),
            Just(Rank::Six),
            Just(Rank::Seven),
            Just(Rank::Eight),
            Just(Rank::Nine),
            Just(Rank::Ten),
            Just(Rank::Jack),
            Just(Rank::Queen),
            Just(Rank::King),
        ]
    }

    fn any_suit() -> impl Strategy<Value = Suit> {
        prop_oneof![
            Just(Suit::Hearts),
            Just(Suit::Diamonds),
            Just(Suit::Clubs),
            Just(Suit::Spades),
        ]
    }

    fn any_card() -> impl Strategy<Value = Card> {
        (any_rank(), any_suit()).prop_map(|(rank, suit)| Card::new(rank, suit))
    }

    // Property: All card values are between 1 and 10
    proptest! {
        #[test]
        fn prop_card_value_in_valid_range(card in any_card()) {
            let value = card.base_value();
            prop_assert!(value >= 1 && value <= 10, "Card value {} out of range [1,10]", value);
        }
    }

    // Property: Card creation preserves rank and suit
    proptest! {
        #[test]
        fn prop_card_creation_preserves_data(rank in any_rank(), suit in any_suit()) {
            let card = Card::new(rank, suit);
            prop_assert_eq!(card.rank(), rank);
            prop_assert_eq!(card.suit(), suit);
        }
    }

    // Property: Cards with same rank and suit are equal
    proptest! {
        #[test]
        fn prop_card_equality_reflexive(rank in any_rank(), suit in any_suit()) {
            let card1 = Card::new(rank, suit);
            let card2 = Card::new(rank, suit);
            prop_assert_eq!(card1, card2);
        }
    }

    // Property: Cards with different rank or suit are not equal
    proptest! {
        #[test]
        fn prop_card_inequality(
            rank1 in any_rank(),
            suit1 in any_suit(),
            rank2 in any_rank(),
            suit2 in any_suit()
        ) {
            let card1 = Card::new(rank1, suit1);
            let card2 = Card::new(rank2, suit2);
            
            if rank1 != rank2 || suit1 != suit2 {
                prop_assert_ne!(card1, card2);
            }
        }
    }

    // Property: Card display always contains " of "
    proptest! {
        #[test]
        fn prop_card_display_format(card in any_card()) {
            let display = format!("{}", card);
            prop_assert!(display.contains(" of "), "Display '{}' doesn't contain ' of '", display);
        }
    }

    // Property: Card copy creates independent copy
    proptest! {
        #[test]
        fn prop_card_copy_works(card in any_card()) {
            let card_copy = card;
            prop_assert_eq!(card.rank(), card_copy.rank());
            prop_assert_eq!(card.suit(), card_copy.suit());
        }
    }

    // Property: Ace always has value 1
    proptest! {
        #[test]
        fn prop_ace_value_is_one(suit in any_suit()) {
            let ace = Card::new(Rank::Ace, suit);
            prop_assert_eq!(ace.base_value(), 1);
        }
    }

    // Property: Face cards (J, Q, K) always have value 10
    proptest! {
        #[test]
        fn prop_face_cards_value_ten(
            face_rank in prop_oneof![Just(Rank::Jack), Just(Rank::Queen), Just(Rank::King)],
            suit in any_suit()
        ) {
            let card = Card::new(face_rank, suit);
            prop_assert_eq!(card.base_value(), 10);
        }
    }

    // Property: Ten card has value 10
    proptest! {
        #[test]
        fn prop_ten_value_is_ten(suit in any_suit()) {
            let ten = Card::new(Rank::Ten, suit);
            prop_assert_eq!(ten.base_value(), 10);
        }
    }

    // Property: Number cards (2-9) have value equal to their rank
    proptest! {
        #[test]
        fn prop_number_cards_match_rank(
            rank in prop_oneof![
                Just((Rank::Two, 2u8)),
                Just((Rank::Three, 3u8)),
                Just((Rank::Four, 4u8)),
                Just((Rank::Five, 5u8)),
                Just((Rank::Six, 6u8)),
                Just((Rank::Seven, 7u8)),
                Just((Rank::Eight, 8u8)),
                Just((Rank::Nine, 9u8)),
            ],
            suit in any_suit()
        ) {
            let (rank_enum, expected_value) = rank;
            let card = Card::new(rank_enum, suit);
            prop_assert_eq!(card.base_value(), expected_value);
        }
    }

    // Structural tests (not property-based)
    #[test]
    fn test_all_ranks_count() {
        let ranks = Rank::all();
        assert_eq!(ranks.len(), 13, "Should have exactly 13 ranks");
    }

    #[test]
    fn test_all_suits_count() {
        let suits = Suit::all();
        assert_eq!(suits.len(), 4, "Should have exactly 4 suits");
    }

    #[test]
    fn test_all_ranks_unique() {
        let ranks = Rank::all();
        for i in 0..ranks.len() {
            for j in (i + 1)..ranks.len() {
                assert_ne!(ranks[i], ranks[j], "Ranks at {} and {} should be different", i, j);
            }
        }
    }

    #[test]
    fn test_all_suits_unique() {
        let suits = Suit::all();
        for i in 0..suits.len() {
            for j in (i + 1)..suits.len() {
                assert_ne!(suits[i], suits[j], "Suits at {} and {} should be different", i, j);
            }
        }
    }
}

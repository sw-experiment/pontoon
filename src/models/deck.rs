use super::card::{Card, Rank, Suit};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Represents a deck of 52 playing cards
pub struct Deck {
    cards: Vec<Card>,
    rng: StdRng,
}

impl Deck {
    /// Creates a new standard 52-card deck and shuffles it
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: Self::create_standard_deck(),
            rng: StdRng::from_entropy(),
        };
        deck.shuffle();
        deck
    }

    /// Creates a new deck with a specific seed (for testing)
    pub fn new_seeded(seed: u64) -> Self {
        let mut deck = Deck {
            cards: Self::create_standard_deck(),
            rng: StdRng::seed_from_u64(seed),
        };
        deck.shuffle();
        deck
    }

    /// Creates a standard 52-card deck (unshuffled)
    fn create_standard_deck() -> Vec<Card> {
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(rank, suit));
            }
        }
        cards
    }

    /// Shuffles the deck using Fisher-Yates algorithm
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }

    /// Deals one card from the top of the deck
    /// Returns None if the deck is empty
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Returns the number of cards remaining in the deck
    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the deck needs reshuffling (< 15 cards)
    pub fn needs_reshuffle(&self) -> bool {
        self.cards.len() < 15
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Property: New deck always has exactly 52 cards
    #[test]
    fn prop_new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards_remaining(), 52);
    }

    // Property: Dealing N cards reduces count by N
    proptest! {
        #[test]
        fn prop_dealing_reduces_count(deal_count in 0usize..=52) {
            let mut deck = Deck::new();
            for _ in 0..deal_count {
                deck.deal();
            }
            prop_assert_eq!(deck.cards_remaining(), 52 - deal_count);
        }
    }

    // Property: Can deal exactly 52 cards, then get None
    proptest! {
        #[test]
        fn prop_can_deal_all_52_cards(seed in any::<u64>()) {
            let mut deck = Deck::new_seeded(seed);
            let mut dealt_cards = Vec::new();
            
            // Deal all 52 cards
            for _ in 0..52 {
                let card = deck.deal();
                prop_assert!(card.is_some(), "Should be able to deal card");
                dealt_cards.push(card.unwrap());
            }
            
            // 53rd deal should return None
            prop_assert_eq!(deck.deal(), None);
            prop_assert_eq!(deck.cards_remaining(), 0);
        }
    }

    // Property: Same seed produces same sequence
    proptest! {
        #[test]
        fn prop_seeded_deck_deterministic(seed in any::<u64>(), deal_count in 1usize..=52) {
            let mut deck1 = Deck::new_seeded(seed);
            let mut deck2 = Deck::new_seeded(seed);
            
            for _ in 0..deal_count {
                let card1 = deck1.deal();
                let card2 = deck2.deal();
                prop_assert_eq!(card1, card2, "Same seed should produce same sequence");
            }
        }
    }

    // Property: needs_reshuffle is true when < 15 cards
    proptest! {
        #[test]
        fn prop_needs_reshuffle_threshold(deal_count in 0usize..=52) {
            let mut deck = Deck::new();
            for _ in 0..deal_count {
                deck.deal();
            }
            
            let remaining = deck.cards_remaining();
            if remaining < 15 {
                prop_assert!(deck.needs_reshuffle(), "Should need reshuffle with {} cards", remaining);
            } else {
                prop_assert!(!deck.needs_reshuffle(), "Should not need reshuffle with {} cards", remaining);
            }
        }
    }

    // Property: All dealt cards are unique (no duplicates in a deck)
    proptest! {
        #[test]
        fn prop_all_cards_unique(seed in any::<u64>()) {
            let mut deck = Deck::new_seeded(seed);
            let mut dealt_cards = Vec::new();
            
            // Deal all cards
            for _ in 0..52 {
                if let Some(card) = deck.deal() {
                    dealt_cards.push(card);
                }
            }
            
            // Check all cards are unique
            for i in 0..dealt_cards.len() {
                for j in (i + 1)..dealt_cards.len() {
                    prop_assert_ne!(
                        dealt_cards[i], 
                        dealt_cards[j],
                        "Found duplicate card at positions {} and {}", i, j
                    );
                }
            }
        }
    }

    // Property: Deck contains all 52 unique cards (13 ranks × 4 suits)
    proptest! {
        #[test]
        fn prop_deck_has_all_cards(seed in any::<u64>()) {
            let mut deck = Deck::new_seeded(seed);
            let mut dealt_cards = Vec::new();
            
            // Deal all cards
            while let Some(card) = deck.deal() {
                dealt_cards.push(card);
            }
            
            prop_assert_eq!(dealt_cards.len(), 52, "Should have exactly 52 cards");
            
            // Check we have all combinations
            for rank in Rank::all() {
                for suit in Suit::all() {
                    let expected_card = Card::new(rank, suit);
                    prop_assert!(
                        dealt_cards.contains(&expected_card),
                        "Missing card: {}", expected_card
                    );
                }
            }
        }
    }

    // Property: Dealing from empty deck always returns None
    proptest! {
        #[test]
        fn prop_empty_deck_always_none(extra_deals in 1usize..=10) {
            let mut deck = Deck::new();
            
            // Empty the deck
            for _ in 0..52 {
                deck.deal();
            }
            
            // Try dealing more times
            for _ in 0..extra_deals {
                prop_assert_eq!(deck.deal(), None, "Empty deck should always return None");
            }
        }
    }

    // Property: Different seeds produce different sequences (check multiple cards)
    proptest! {
        #[test]
        fn prop_different_seeds_different_shuffle(seed1 in any::<u64>(), seed2 in any::<u64>()) {
            prop_assume!(seed1 != seed2); // Only test when seeds are different
            
            let mut deck1 = Deck::new_seeded(seed1);
            let mut deck2 = Deck::new_seeded(seed2);
            
            // Compare first 5 cards - if all match, shuffling is broken
            let mut all_match = true;
            for _ in 0..5 {
                let card1 = deck1.deal().unwrap();
                let card2 = deck2.deal().unwrap();
                if card1 != card2 {
                    all_match = false;
                    break;
                }
            }
            
            // Probability of 5 cards matching by chance is (1/52)^5 ≈ 1 in 380 million
            prop_assert!(!all_match, "Different seeds produced identical first 5 cards - shuffling may be broken");
        }
    }
}

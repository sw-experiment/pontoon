use super::card::Card;

/// Represents a hand of cards
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new empty hand
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    /// Adds a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Returns the number of cards in the hand
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// Returns an immutable reference to the cards in the hand
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Clears all cards from the hand
    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::{Rank, Suit};
    use proptest::prelude::*;

    // Generator for any card
    fn any_card() -> impl Strategy<Value = Card> {
        let ranks = vec![
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ];
        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        
        (0..13usize, 0..4usize).prop_map(move |(r, s)| {
            Card::new(ranks[r], suits[s])
        })
    }

    // Property: New hand is always empty
    #[test]
    fn prop_new_hand_empty() {
        let hand = Hand::new();
        assert_eq!(hand.card_count(), 0);
        assert_eq!(hand.cards().len(), 0);
    }

    // Property: Adding N cards results in count of N
    proptest! {
        #[test]
        fn prop_add_cards_increases_count(cards in prop::collection::vec(any_card(), 0..20)) {
            let mut hand = Hand::new();
            
            for (i, card) in cards.iter().enumerate() {
                hand.add_card(*card);
                prop_assert_eq!(hand.card_count(), i + 1);
            }
            
            prop_assert_eq!(hand.card_count(), cards.len());
        }
    }

    // Property: Cards are stored in order added
    proptest! {
        #[test]
        fn prop_cards_stored_in_order(cards in prop::collection::vec(any_card(), 1..20)) {
            let mut hand = Hand::new();
            
            for card in &cards {
                hand.add_card(*card);
            }
            
            let stored_cards = hand.cards();
            prop_assert_eq!(stored_cards.len(), cards.len());
            
            for (i, card) in cards.iter().enumerate() {
                prop_assert_eq!(stored_cards[i], *card, "Card at position {} doesn't match", i);
            }
        }
    }

    // Property: Clear always results in empty hand
    proptest! {
        #[test]
        fn prop_clear_empties_hand(cards in prop::collection::vec(any_card(), 0..20)) {
            let mut hand = Hand::new();
            
            for card in cards {
                hand.add_card(card);
            }
            
            hand.clear();
            
            prop_assert_eq!(hand.card_count(), 0);
            prop_assert_eq!(hand.cards().len(), 0);
        }
    }

    // Property: Can add same card multiple times (e.g., from multiple decks)
    proptest! {
        #[test]
        fn prop_can_add_duplicate_cards(card in any_card(), count in 1usize..10) {
            let mut hand = Hand::new();
            
            for _ in 0..count {
                hand.add_card(card);
            }
            
            prop_assert_eq!(hand.card_count(), count);
            
            // All cards should be the same
            for stored_card in hand.cards() {
                prop_assert_eq!(*stored_card, card);
            }
        }
    }

    // Property: cards() returns immutable reference (can't modify through it)
    proptest! {
        #[test]
        fn prop_cards_returns_immutable_ref(cards in prop::collection::vec(any_card(), 1..10)) {
            let mut hand = Hand::new();
            
            for card in &cards {
                hand.add_card(*card);
            }
            
            let cards_ref1 = hand.cards();
            let cards_ref2 = hand.cards();
            
            // Both references should point to same data
            prop_assert_eq!(cards_ref1.len(), cards_ref2.len());
            for i in 0..cards_ref1.len() {
                prop_assert_eq!(cards_ref1[i], cards_ref2[i]);
            }
        }
    }

    // Property: Hand can hold many cards (no arbitrary limit)
    proptest! {
        #[test]
        fn prop_hand_can_hold_many_cards(count in 1usize..100) {
            let mut hand = Hand::new();
            let card = Card::new(Rank::Ace, Suit::Hearts);
            
            for _ in 0..count {
                hand.add_card(card);
            }
            
            prop_assert_eq!(hand.card_count(), count);
        }
    }

    // Property: Clear is idempotent (clearing twice has same effect as once)
    proptest! {
        #[test]
        fn prop_clear_idempotent(cards in prop::collection::vec(any_card(), 1..10)) {
            let mut hand = Hand::new();
            
            for card in cards {
                hand.add_card(card);
            }
            
            hand.clear();
            let count_after_first_clear = hand.card_count();
            
            hand.clear();
            let count_after_second_clear = hand.card_count();
            
            prop_assert_eq!(count_after_first_clear, count_after_second_clear);
            prop_assert_eq!(count_after_second_clear, 0);
        }
    }
}

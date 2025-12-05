use pontoon::models::deck::Deck;
use pontoon::models::hand::Hand;
use pontoon::ui::display::Display;

fn main() {
    // Create display
    let display = Display::new();
    
    // Show welcome
    display.show_welcome();
    display.show_message("Welcome to Pontoon!");
    display.show_message("Dealing initial cards...");
    
    // Create and shuffle deck
    let mut deck = Deck::new();
    
    // Create hands for player and banker
    let mut player_hand = Hand::new();
    let mut banker_hand = Hand::new();
    
    // Deal initial cards (2 to each)
    player_hand.add_card(deck.deal().expect("Deck should have cards"));
    banker_hand.add_card(deck.deal().expect("Deck should have cards"));
    player_hand.add_card(deck.deal().expect("Deck should have cards"));
    banker_hand.add_card(deck.deal().expect("Deck should have cards"));
    
    // Display hands
    display.show_separator();
    display.show_player_hand(&player_hand);
    display.show_banker_hand_hidden(&banker_hand);
    display.show_separator();
    
    display.show_message(&format!("\nCards remaining in deck: {}", deck.cards_remaining()));
    display.show_message("\n[Story 1 Demo Complete - Initial Deal Working!]");
}

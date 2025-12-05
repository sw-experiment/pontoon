use crate::models::hand::Hand;

/// Handles all game output and formatting
pub struct Display;

impl Display {
    pub fn new() -> Self {
        Display
    }

    /// Shows the welcome message and game title
    pub fn show_welcome(&self) {
        println!("\n╔═══════════════════════════════════════╗");
        println!("║                                       ║");
        println!("║            PONTOON GAME               ║");
        println!("║      British Card Game Classic        ║");
        println!("║                                       ║");
        println!("╚═══════════════════════════════════════╝\n");
    }

    /// Shows the player's hand
    pub fn show_player_hand(&self, hand: &Hand) {
        println!("\n┌─ Your Hand ─────────────────────────┐");
        for card in hand.cards() {
            println!("│  {}", card);
        }
        println!("└─────────────────────────────────────┘");
    }

    /// Shows the banker's hand with one card hidden
    pub fn show_banker_hand_hidden(&self, hand: &Hand) {
        println!("\n┌─ Banker's Hand ─────────────────────┐");
        if let Some(first_card) = hand.cards().first() {
            println!("│  {}", first_card);
        }
        if hand.card_count() > 1 {
            println!("│  [Hidden Card]");
        }
        println!("└─────────────────────────────────────┘");
    }

    /// Shows a message
    pub fn show_message(&self, message: &str) {
        println!("\n{}", message);
    }

    /// Shows a separator line
    pub fn show_separator(&self) {
        println!("\n═══════════════════════════════════════════");
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

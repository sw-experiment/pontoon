# Design Document

## Overview

The Pontoon game application is a command-line card game implemented in Rust that allows a human player to play British Pontoon against a computer banker. The design leverages Rust's type system and ownership model to guarantee memory safety, prevent null pointer errors, and ensure correctness by construction. The architecture follows a modular design with clear separation of concerns between game logic, user interface, and data models.

The system is designed to meet all ISO 25010 NFRs through language-level guarantees: reliability through Rust's safety features, performance through zero-cost abstractions, security through memory safety, and maintainability through strong typing. The architecture supports authentic British Pontoon rules including special hands (Pontoon, Five Card Trick) and proper hand evaluation hierarchy.

## Architecture

### High-Level Architecture

The application follows a layered architecture pattern:

```
┌─────────────────────────────────────┐
│     Presentation Layer (CLI)        │
│  - Display formatting               │
│  - Input handling                   │
│  - User interaction                 │
└─────────────────────────────────────┘
              ↓ ↑
┌─────────────────────────────────────┐
│      Game Controller Layer          │
│  - Game flow orchestration          │
│  - Round management                 │
│  - Turn coordination                │
└─────────────────────────────────────┘
              ↓ ↑
┌─────────────────────────────────────┐
│       Domain Logic Layer            │
│  - Card and Deck models             │
│  - Hand evaluation                  │
│  - Pontoon rules engine             │
│  - Banker strategy                  │
└─────────────────────────────────────┘
```

### Module Structure

```
pontoon/
├── Cargo.toml              # Project manifest and dependencies
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library root
│   ├── models/
│   │   ├── mod.rs
│   │   ├── card.rs        # Card struct
│   │   ├── deck.rs        # Deck struct
│   │   └── hand.rs        # Hand struct with evaluation
│   ├── game/
│   │   ├── mod.rs
│   │   ├── controller.rs  # Main game loop and flow
│   │   ├── rules.rs       # Pontoon rules and hand comparison
│   │   └── banker.rs      # Banker decision logic
│   └── ui/
│       ├── mod.rs
│       ├── display.rs     # Output formatting
│       └── input.rs       # Input validation and parsing
└── tests/
    ├── card_tests.rs
    ├── deck_tests.rs
    ├── hand_tests.rs
    ├── rules_tests.rs
    ├── banker_tests.rs
    └── integration_tests.rs
```

## Components and Interfaces

### 1. Card Model (`models/card.rs`)

**Purpose**: Represents a single playing card with rank and suit.

**Struct: Card**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts, Diamonds, Clubs, Spades
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self
    pub fn base_value(&self) -> u8  // Returns base value (Ace=1, Face=10)
    pub fn rank(&self) -> Rank
    pub fn suit(&self) -> Suit
}

impl Display for Card  // "Rank of Suit"
```

**Key Behaviors**:
- Immutable by default (Rust's ownership model)
- Copy semantics (cards are small, cheap to copy)
- Type-safe enums prevent invalid ranks/suits
- No null/undefined states possible

### 2. Deck Model (`models/deck.rs`)

**Purpose**: Manages a collection of 52 cards with shuffling and dealing.

**Struct: Deck**
```rust
pub struct Deck {
    cards: Vec<Card>,
    rng: StdRng,  // Seeded RNG for deterministic testing
}

impl Deck {
    pub fn new() -> Self
    pub fn new_seeded(seed: u64) -> Self  // For testing
    pub fn shuffle(&mut self)
    pub fn deal(&mut self) -> Option<Card>  // Returns None if empty
    pub fn cards_remaining(&self) -> usize
    pub fn needs_reshuffle(&self) -> bool  // True if < 15 cards
}
```

**Key Behaviors**:
- Creates standard 52-card deck on initialization
- Shuffles using Fisher-Yates algorithm (via rand crate)
- Returns Option<Card> - no null pointers, explicit empty handling
- Mutable methods require &mut self (ownership prevents concurrent modification)
- Supports seeding for deterministic testing

### 3. Hand Model (`models/hand.rs`)

**Purpose**: Represents a player's or banker's hand with card collection and value calculation.

**Struct: Hand**
```rust
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self
    pub fn add_card(&mut self, card: Card)
    pub fn value(&self) -> u8  // Optimal value with Ace handling
    pub fn is_bust(&self) -> bool
    pub fn is_pontoon(&self) -> bool
    pub fn is_five_card_trick(&self) -> bool
    pub fn card_count(&self) -> usize
    pub fn cards(&self) -> &[Card]  // Immutable borrow
    pub fn clear(&mut self)
}
```

**Key Behaviors**:
- Calculates optimal hand value (Aces as 11 when beneficial, else 1)
- Detects special hands (Pontoon, Five Card Trick)
- Determines bust status (value > 21)
- Immutable borrows prevent accidental modification
- No null cards possible (Vec guarantees valid elements)

**Value Calculation Algorithm**:
1. Count Aces in hand
2. Calculate base value (all Aces as 1)
3. Try to use one Ace as 11 if it doesn't cause bust
4. Return optimal value (u8 - can't overflow, max is 11*4 + 10*9 = 134, but practical max is 21)

### 4. Rules Engine (`game/rules.py`)

**Purpose**: Implements Pontoon rules and hand comparison logic.

**Functions**:
```python
def compare_hands(player_hand: Hand, banker_hand: Hand) -> str
    """
    Compares hands and returns result: 'player_wins', 'banker_wins', 'push'
    
    Hand hierarchy:
    1. Pontoon (21 with 2 cards)
    2. Five Card Trick (5 cards without bust)
    3. 21 or less (higher wins)
    """

def get_hand_type(hand: Hand) -> str
    """Returns hand type: 'pontoon', 'five_card_trick', 'bust', 'normal'"""

def get_hand_rank(hand: Hand) -> Tuple[int, int]
    """Returns (type_rank, value) for comparison"""
```

**Hand Comparison Rules**:
1. Pontoon beats everything except banker's Pontoon
2. Five Card Trick beats everything except Pontoon
3. Higher value beats lower value (if same type)
4. Ties go to banker (house advantage)
5. Bust always loses

### 5. Banker Strategy (`game/banker_strategy.py`)

**Purpose**: Implements banker's decision-making logic.

**Class: BankerStrategy**
```python
class BankerStrategy:
    """Implements banker's playing strategy."""
    
    def should_twist(self, hand: Hand) -> bool
        """Returns True if banker should twist, False if should stick"""
```

**Strategy Rules**:
- Twist if hand value ≤ 16
- Stick if hand value ≥ 17
- Always stick on Five Card Trick
- Always stick on Pontoon

### 6. Game Controller (`game/game_controller.py`)

**Purpose**: Orchestrates game flow, manages rounds, and coordinates between components.

**Class: GameController**
```python
class GameController:
    """Main game controller managing game flow."""
    
    def __init__(self, display: Display, input_handler: InputHandler)
    def start_game(self) -> None
    def play_round(self) -> None
    def player_turn(self) -> bool  # Returns True if player didn't bust
    def banker_turn(self) -> None
    def determine_winner(self) -> str
    def update_statistics(self, result: str) -> None
    def show_statistics(self) -> None
```

**Attributes**:
- `deck`: Deck instance
- `player_hand`: Hand instance
- `banker_hand`: Hand instance
- `display`: Display instance
- `input_handler`: InputHandler instance
- `statistics`: Dict tracking wins/losses/rounds

**Game Flow**:
1. Initialize game (welcome, instructions)
2. Start round (shuffle if needed, deal cards)
3. Player turn (twist/stick loop)
4. Banker turn (if player didn't bust)
5. Determine winner
6. Update statistics
7. Ask to play again or quit

### 7. Display (`ui/display.py`)

**Purpose**: Handles all output formatting and display to terminal.

**Class: Display**
```python
class Display:
    """Handles all game output and formatting."""
    
    def show_welcome(self) -> None
    def show_instructions(self) -> None
    def show_player_hand(self, hand: Hand, show_all: bool = True) -> None
    def show_banker_hand(self, hand: Hand, hide_first: bool = True) -> None
    def show_both_hands(self, player_hand: Hand, banker_hand: Hand) -> None
    def show_result(self, result: str, player_hand: Hand, banker_hand: Hand) -> None
    def show_statistics(self, stats: Dict) -> None
    def show_message(self, message: str) -> None
    def show_error(self, error: str) -> None
    def clear_screen(self) -> None  # Optional, for cleaner display
```

**Display Formatting**:
- Clear visual separation between sections
- Consistent card display format
- Hidden card shown as "[Hidden]"
- Hand values displayed prominently
- Special hands (Pontoon, Five Card Trick) highlighted

### 8. Input Handler (`ui/input_handler.py`)

**Purpose**: Handles user input with validation and error handling.

**Class: InputHandler**
```python
class InputHandler:
    """Handles user input with validation."""
    
    def get_player_action(self) -> str
        """Prompts for and validates player action (twist/stick)"""
    
    def get_yes_no(self, prompt: str) -> bool
        """Prompts for yes/no response"""
    
    def _validate_action(self, input_str: str) -> Optional[str]
        """Validates and normalizes action input"""
```

**Input Validation**:
- Accepts full words: "twist", "stick"
- Accepts shortcuts: "t", "s"
- Case-insensitive
- Handles whitespace
- Provides helpful error messages for invalid input

## Data Models

### Card Value Mapping

```python
RANK_VALUES = {
    'A': 1,   # Ace (can be 1 or 11)
    '2': 2, '3': 3, '4': 4, '5': 5,
    '6': 6, '7': 7, '8': 8, '9': 9, '10': 10,
    'J': 10, 'Q': 10, 'K': 10
}

SUITS = ['Hearts', 'Diamonds', 'Clubs', 'Spades']
RANKS = ['A', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K']
```

### Game State

```python
@dataclass
class GameStatistics:
    """Tracks game statistics."""
    rounds_played: int = 0
    player_wins: int = 0
    banker_wins: int = 0
    player_pontoons: int = 0
    player_five_card_tricks: int = 0
```

### Hand Type Rankings

```python
HAND_TYPE_RANKS = {
    'pontoon': 4,
    'five_card_trick': 3,
    'normal': 2,
    'bust': 1
}
```

## Error Handling

### Error Categories

1. **User Input Errors**
   - Invalid action input
   - Unexpected input format
   - Handled by: InputHandler with re-prompting

2. **Game State Errors**
   - Empty deck (should never happen with reshuffle logic)
   - Invalid hand state
   - Handled by: Defensive programming and assertions

3. **System Errors**
   - File I/O errors (if logging)
   - Memory errors
   - Handled by: Top-level try-catch in main.py

### Error Handling Strategy

```python
# Input errors: Catch and re-prompt
try:
    action = input_handler.get_player_action()
except ValueError as e:
    display.show_error(str(e))
    # Re-prompt

# Game logic: Use assertions for impossible states
assert deck.cards_remaining() > 0, "Deck should have been reshuffled"

# System errors: Log and exit gracefully
try:
    game_controller.start_game()
except Exception as e:
    logging.error(f"Critical error: {e}")
    print("An unexpected error occurred. Please restart the game.")
    sys.exit(1)
```

## Testing Strategy

### Unit Testing

**Test Coverage Goals**: 80%+ code coverage

**Key Test Suites**:

1. **Card Tests** (`test_card.py`)
   - Card creation with valid ranks/suits
   - Value calculation for all ranks
   - String representation
   - Invalid card creation

2. **Deck Tests** (`test_deck.py`)
   - Deck initialization (52 cards)
   - Shuffling (deterministic with seed)
   - Dealing cards
   - Reshuffle detection
   - Empty deck handling

3. **Hand Tests** (`test_hand.py`)
   - Adding cards
   - Value calculation with Aces
   - Pontoon detection
   - Five Card Trick detection
   - Bust detection
   - Edge cases (multiple Aces)

4. **Rules Tests** (`test_rules.py`)
   - Hand comparison for all scenarios
   - Hand type detection
   - Hand ranking
   - Tie-breaking (banker wins)

5. **Banker Strategy Tests** (`test_banker_strategy.py`)
   - Twist on 16 or below
   - Stick on 17 or above
   - Special hand handling

6. **Game Controller Tests** (`test_game_controller.py`)
   - Round flow
   - Turn management
   - Statistics tracking
   - Deck reshuffling

### Integration Testing

**Test Scenarios**:
1. Complete game round (player wins)
2. Complete game round (banker wins)
3. Player busts
4. Banker busts
5. Pontoon scenarios
6. Five Card Trick scenarios
7. Multiple rounds with deck reshuffle

### Test Data

```python
# Example test fixtures
@pytest.fixture
def sample_deck():
    return Deck(seed=42)  # Deterministic for testing

@pytest.fixture
def pontoon_hand():
    hand = Hand()
    hand.add_card(Card('A', 'Hearts'))
    hand.add_card(Card('K', 'Spades'))
    return hand

@pytest.fixture
def five_card_trick_hand():
    hand = Hand()
    for rank in ['2', '3', '4', '5', '6']:
        hand.add_card(Card(rank, 'Hearts'))
    return hand
```

### Performance Testing

**Performance Benchmarks**:
- Deck shuffle: < 50ms
- Hand value calculation: < 10ms
- Complete round: < 100ms
- Application startup: < 1s

**Testing Approach**:
```python
import time

def test_shuffle_performance():
    deck = Deck()
    start = time.time()
    deck.shuffle()
    duration = time.time() - start
    assert duration < 0.05, f"Shuffle took {duration}s, expected < 0.05s"
```

## Implementation Notes

### Ace Handling Algorithm

The most complex logic is handling Aces (value 1 or 11):

```python
def get_value(self) -> int:
    """Calculate optimal hand value."""
    value = sum(card.get_value() for card in self.cards)
    aces = sum(1 for card in self.cards if card.rank == 'A')
    
    # Try to use one Ace as 11
    while aces > 0 and value + 10 <= 21:
        value += 10
        aces -= 1
    
    return value
```

### Deterministic Testing

For reproducible tests, the Deck class accepts an optional seed:

```python
def __init__(self, seed: Optional[int] = None):
    self._seed = seed
    self.cards = self._create_deck()
    self.shuffle()

def shuffle(self):
    if self._seed is not None:
        random.seed(self._seed)
    random.shuffle(self.cards)
```

### Cross-Platform Compatibility

- Use `os.linesep` for line endings
- Use `pathlib` for any file paths
- Avoid platform-specific terminal commands
- Test on Windows, macOS, and Linux

### Performance Optimization

- Use list comprehensions where appropriate
- Avoid unnecessary object creation
- Cache hand values if recalculated frequently (not needed for this simple game)
- Use efficient shuffling algorithm (Fisher-Yates)

## Dependencies

**Required**:
- Rust 1.70+ (2021 edition)
- `rand` crate (for shuffling with seeded RNG)
- Standard library

**Development/Testing**:
- `cargo test` (built-in testing)
- `cargo clippy` (linting)
- `cargo fmt` (formatting)
- `cargo tarpaulin` or `cargo-llvm-cov` (coverage)

**Cargo.toml**:
```toml
[package]
name = "pontoon"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"

[dev-dependencies]
# Testing dependencies if needed
```

## Configuration

**No configuration files needed** - the game runs with sensible defaults.

Optional future enhancements could include:
- Config file for display preferences
- Difficulty levels for banker strategy
- Custom deck sizes or rules variants

## Rust-Specific Design Benefits

### Memory Safety
- No null pointer dereferences (Option<T> instead)
- No use-after-free (ownership system)
- No data races (borrow checker)

### Type Safety
- Invalid card ranks/suits impossible (enums)
- Hand value overflow prevented (u8 type)
- Exhaustive pattern matching enforced

### Performance
- Zero-cost abstractions
- No garbage collection pauses
- Stack allocation for cards (Copy trait)
- Single binary deployment

## Platform Support

### Phase 1: Windows Only
- Target: `x86_64-pc-windows-gnu` (MinGW toolchain)
- Single Windows executable (.exe)
- No Visual Studio required

### Phase 2: Cross-Platform (TODO)
- Additional targets: Linux (`x86_64-unknown-linux-gnu`), macOS (`x86_64-apple-darwin`)
- Cross-compilation or native builds per platform
- CI/CD pipeline for multi-platform releases

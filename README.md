# Pontoon - British Card Game

A command-line implementation of the British card game Pontoon (similar to Blackjack) written in Rust.

## About Pontoon

Pontoon is a British card game descended from the French Vingt-Un. Players compete against a banker to get as close to 21 as possible without going over (busting).

### Special Hands
- **Pontoon**: 21 with exactly 2 cards (Ace + 10-value card) - best hand
- **Five Card Trick**: 5 cards without busting - second best hand

### Rules
- Banker must twist on 16 or below, stick on 17 or above
- Ties go to the banker (house advantage)
- Hand hierarchy: Pontoon > Five Card Trick > 21 > lower totals

## Installation

### Prerequisites
- Rust 1.70 or higher

### Building for All Platforms

**Windows (PowerShell):**
```powershell
.\build.ps1
```

**Linux/macOS (Bash):**
```bash
./build.sh
```

The build script will:
- Cross-compile for Windows, macOS, and Linux (Requirement 11.1)
- Run tests on your current platform
- Generate binaries in `target/<platform>/release/`

### Quick Build (Current Platform Only)
```bash
cargo build --release
```

### Running
```bash
# Windows
.\target\release\pontoon.exe

# Linux/macOS
./target/release/pontoon
```

### Testing
```bash
cargo test
```

## Project Structure

```
pontoon/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library root
│   ├── models/           # Data models (Card, Deck, Hand)
│   ├── game/             # Game logic (rules, banker strategy)
│   └── ui/               # User interface (display, input)
├── tests/                # Integration tests
└── Cargo.toml            # Project manifest
```

## Development Status

- [x] **Story 1: Game Setup and Initial Deal** ✓
  - Card model with type-safe Rank and Suit enums
  - Deck with shuffling and dealing
  - Hand model for holding cards
  - Display module for formatted output
  - Working demo: Shows welcome and deals initial cards
  
- [ ] Story 2: Player Turn (Twist/Stick)
- [ ] Story 3: Banker Turn and Winner
- [ ] Story 4: Special Hands (Pontoon, Five Card Trick)
- [ ] Story 5: Multiple Rounds and Statistics
- [ ] Story 6: Error Handling
- [ ] Story 7: Polish and NFR Verification

## Design Principles

This project prioritizes:
- **Correctness by construction**: Rust's type system prevents entire bug classes
- **Memory safety**: No null pointers, no use-after-free
- **Performance**: Zero-cost abstractions, native performance
- **Reliability**: Comprehensive testing, 80%+ coverage goal
- **Maintainability**: Clear code structure, strong typing

## License

MIT

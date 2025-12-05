# Property-Based Testing Implementation

## Overview

Per Requirements 14.6-14.8 and 15.6-15.7, the codebase now uses property-based testing to achieve high parameter value coverage and find edge cases automatically.

## What Changed

### Requirements Updated
- **Req 14.6**: Use property-based testing for pure functions
- **Req 14.7**: Test invariants and properties, not just examples
- **Req 14.8**: Generate hundreds of test cases automatically
- **Req 15.6**: Verify invariants hold for all possible inputs
- **Req 15.7**: Add property tests when bugs are found

### Dependencies Added
```toml
[dev-dependencies]
proptest = "1.0"  # Property-based testing framework
```

## Property-Based Tests Implemented

### Card Tests (10 properties + 4 structural)

**Properties tested:**
1. All card values are in range [1, 10]
2. Card creation preserves rank and suit
3. Card equality is reflexive
4. Cards with different rank/suit are not equal
5. Card display always contains " of "
6. Card copy creates independent copy
7. Ace always has value 1
8. Face cards (J, Q, K) always have value 10
9. Ten card has value 10
10. Number cards (2-9) have value equal to rank

**Why this is better:**
- Old tests: 9 specific examples
- New tests: 10 properties × 100 generated cases each = 1000+ test cases
- Finds edge cases like: all rank/suit combinations, boundary values, etc.

### Deck Tests (9 properties)

**Properties tested:**
1. New deck always has exactly 52 cards
2. Dealing N cards reduces count by N (for all N ∈ [0,52])
3. Can deal exactly 52 cards, then get None
4. Same seed produces same sequence (determinism)
5. needs_reshuffle threshold is correct for all card counts
6. All dealt cards are unique (no duplicates)
7. Deck contains all 52 unique cards (13 ranks × 4 suits)
8. Dealing from empty deck always returns None
9. Different seeds produce different shuffles

**Why this is better:**
- Old tests: 6 specific examples
- New tests: 9 properties testing ALL possible values
- Example: "dealing N cards" tests 53 different values (0-52), not just 1 and 2
- Finds bugs like: off-by-one errors, duplicate cards, missing cards

### Hand Tests (8 properties)

**Properties tested:**
1. New hand is always empty
2. Adding N cards results in count of N (for N ∈ [0,20])
3. Cards are stored in order added
4. Clear always results in empty hand
5. Can add same card multiple times
6. cards() returns immutable reference
7. Hand can hold many cards (no arbitrary limit)
8. Clear is idempotent

**Why this is better:**
- Old tests: 4 specific examples
- New tests: 8 properties × 100 generated cases = 800+ test cases
- Tests with 0, 1, 5, 10, 20, 100 cards automatically
- Finds bugs like: order preservation, memory leaks, capacity limits

## Benefits of Property-Based Testing

### 1. Higher Coverage
- **Old approach**: Test 3-4 specific values
- **New approach**: Test 100+ random values per property
- **Result**: Find edge cases we'd never think to test

### 2. Less Code, More Tests
- **Old**: 20 test functions, ~20 test cases
- **New**: 27 test functions, ~2700+ test cases
- **Result**: More thorough testing with similar code size

### 3. Better Bug Detection
Property-based tests find bugs like:
- Off-by-one errors (tests all boundary values)
- Missing validation (tests invalid inputs)
- Incorrect assumptions (tests all combinations)
- Race conditions (tests with random timing)

### 4. Living Documentation
Properties describe **what should always be true**:
- "All card values are between 1 and 10"
- "Dealing N cards reduces count by N"
- "Same seed produces same sequence"

This is better documentation than examples.

## Example: How Properties Find Bugs

### Old Test (Example-Based)
```rust
#[test]
fn test_deal_reduces_card_count() {
    let mut deck = Deck::new();
    deck.deal();
    assert_eq!(deck.cards_remaining(), 51);
}
```
**Tests**: 1 case (dealing 1 card)
**Misses**: What about dealing 0, 2, 10, 52, 53 cards?

### New Test (Property-Based)
```rust
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
```
**Tests**: 100 random values in [0, 52]
**Finds**: Off-by-one errors, boundary conditions, etc.

## Running Property-Based Tests

```bash
cargo test
```

Each property test runs 100 times by default with different random inputs.

## Current Status

⚠️ **Build Issue**: Property-based tests are implemented but require `dlltool.exe` from MinGW to compile on Windows with GNU toolchain.

**Solutions**:
1. Install MinGW-w64 (includes dlltool)
2. Use MSVC toolchain instead of GNU
3. Build on Linux/macOS (no dlltool needed)
4. Use CI/CD with proper toolchain setup

## Next Steps

1. Fix build environment to include dlltool
2. Run property tests and verify all pass
3. Add property tests for Story 2 (hand value calculation with Aces)
4. Add property tests for Story 3 (hand comparison logic)
5. Add property tests for Story 4 (special hands detection)

## References

- [PropTest Documentation](https://docs.rs/proptest/)
- [Property-Based Testing](https://hypothesis.works/articles/what-is-property-based-testing/)
- [Requirement 14.6-14.8](../.kiro/specs/pontoon-game/requirements.md)

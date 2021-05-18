# SpellingBeeSolver

Simple CLI written in rust to generate solutions to the [NYT Spelling Bee puzzles](https://www.nytimes.com/puzzles/spelling-bee).

Defaults to using the built-in word dictionary, which is much more expansive than the dictionary the NYT uses. That is, this will generate a bunch of solutions that aren't recognized as valid. Similarly, there might be words that NYT is aware of that the words dictionary can't find.

## Usage

```sh
spelling_bee_solver <primary_character> <supporting_characters>
```

Where the primary character is the center, "yellow" character that much be present in all words. The support characters is a string (no spaces, not a list) of all other characters allowed by the puzzle.

## Installing

- [set up cargo and rust](https://rustup.rs)
- run `cargo install spelling_bee_solver`

## Future Work

Build a robot to enter these solutions in.

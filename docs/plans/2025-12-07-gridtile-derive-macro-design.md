# GridTile Derive Macro Design

**Date:** 2025-12-07
**Status:** Approved

## Overview

A procedural derive macro that automates the boilerplate for grid-based Advent of Code puzzles. Generates `Debug`, `Display`, and a nom parser for enum types representing grid tiles.

## Motivation

Current pattern across day04, day07, and other grid-based solutions:
- Define `Tile` enum with variants for cell types
- Manually implement `Debug` showing character representation
- Manually implement `Display` with identical logic
- Manually write nom parser mapping characters to variants

This is repetitive boilerplate with identical structure across solutions.

## Design

### Project Structure

```
advent/
├── common/                 # Existing utilities
├── common_derive/          # New proc macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── 2025/day04/             # Usage site
```

### Usage

```rust
use common_derive::GridTile;

#[derive(Copy, Clone, PartialEq, Eq, GridTile)]
enum Tile {
    #[tile('@')]
    Roll,
    #[tile('.')]
    Empty,
}

fn parse(input: &str) -> Grid<Tile> {
    let result: IResult<&str, Grid<Tile>> = parse_grid(Tile::parser()).parse(input);
    result.unwrap().1
}
```

### Generated Code

The macro generates three implementations:

#### 1. Debug Implementation
```rust
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Roll => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}
```

#### 2. Display Implementation
```rust
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Roll => write!(f, "@"),
            Tile::Empty => write!(f, "."),
        }
    }
}
```

#### 3. Parser Method
```rust
impl Tile {
    pub fn parser<I, E>() -> impl nom::Parser<I, Output = Self, Error = E>
    where
        I: Clone + nom::Input,
        E: nom::error::ParseError<I>,
        <I as nom::Input>::Item: nom::AsChar,
    {
        nom::branch::alt((
            nom::combinator::map(nom::character::complete::char('@'), |_| Tile::Roll),
            nom::combinator::map(nom::character::complete::char('.'), |_| Tile::Empty),
        ))
    }
}
```

### Macro Implementation Details

**Crate setup:**
- New `common_derive` crate with `proc-macro = true`
- Dependencies: `syn`, `quote`, `proc-macro2`

**Macro logic:**
1. Parse the enum definition using `syn`
2. Extract `#[tile('c')]` attributes from each variant
3. Validate all variants have exactly one `#[tile]` attribute
4. Generate match arms for Debug/Display using the characters
5. Generate nom parser using `alt()` combinator with one branch per variant
6. Use `quote!` to generate the implementation code

**Error handling:**
- Missing `#[tile]` attribute on variant → compile error with helpful message
- Invalid character literal in `#[tile]` → compile error
- Non-enum type → compile error

### Integration with Workspace

**Workspace Cargo.toml:**
```toml
[workspace]
members = [
  "common",
  "common_derive",  # Add this
  "2015/day*",
  # ... rest unchanged
]
```

**common/Cargo.toml:**
```toml
[dependencies]
common_derive = { path = "../common_derive" }  # Re-export for convenience
```

**common/lib.rs:**
```rust
pub use common_derive::GridTile;  // Re-export so users import from common
```

This allows usage sites to write:
```rust
use common::GridTile;  // or
use common_derive::GridTile;  // both work
```

### Migration Path

The macro is additive and non-breaking:
- Existing solutions continue to work unchanged
- New solutions can use `#[derive(GridTile)]`
- Old solutions can be migrated incrementally when convenient

## Implementation Tasks

1. Create `common_derive` crate structure
2. Implement derive macro parsing and validation
3. Generate Debug implementation
4. Generate Display implementation
5. Generate parser() method
6. Add to workspace and re-export from common
7. Test with day04 as reference implementation
8. Migrate day07 to validate across different tile sets

## Trade-offs

**Benefits:**
- Eliminates ~30 lines of boilerplate per solution
- Ensures consistency across Debug/Display/parser
- Better IDE support than declarative macro
- Feels like idiomatic Rust with standard derive syntax

**Costs:**
- New proc macro crate adds build complexity
- Attribute syntax is macro-specific (not discoverable without docs)
- Generated code is less visible than hand-written

The benefits outweigh costs for this repetitive pattern across 100+ Advent of Code solutions.

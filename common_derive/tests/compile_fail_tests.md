# Compile-Fail Tests

These tests verify that the macro correctly rejects invalid inputs. They are documented here but not run automatically (would require trybuild or similar).

## Test 1: Non-unit variants should fail

```rust
use common_derive::GridTile;

#[derive(GridTile)]
enum BadTile {
    #[tile('@')]
    Roll(u32),  // This should fail - not a unit variant
    #[tile('.')]
    Empty,
}
```

Expected error: "GridTile only supports unit variants (no fields)"

## Test 2: Missing tile attribute should fail

```rust
use common_derive::GridTile;

#[derive(GridTile)]
enum BadTile {
    Roll,  // Missing #[tile('c')] attribute
    #[tile('.')]
    Empty,
}
```

Expected error: "missing #[tile('c')] attribute"

## Test 3: Non-enum types should fail

```rust
use common_derive::GridTile;

#[derive(GridTile)]
struct NotAnEnum {
    field: u32,
}
```

Expected error: "GridTile can only be derived for enums"

## Test 4: Invalid character literal should fail

```rust
use common_derive::GridTile;

#[derive(GridTile)]
enum BadTile {
    #[tile("string")]  // Should be char, not string
    Roll,
}
```

Expected error: "tile attribute must contain a character literal"

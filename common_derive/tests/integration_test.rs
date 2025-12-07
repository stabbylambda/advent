use common_derive::GridTile;

#[derive(GridTile)]
enum Tile {
    #[tile('@')]
    Roll,
    #[tile('.')]
    Empty,
}

#[test]
fn test_compiles() {
    // If this compiles, our macro is working correctly
    // In future tasks, we'll test the generated functionality
}

#[test]
fn test_debug_implementation() {
    // Test that Debug trait is implemented and formats correctly
    let roll = Tile::Roll;
    let empty = Tile::Empty;

    assert_eq!(format!("{:?}", roll), "@");
    assert_eq!(format!("{:?}", empty), ".");
}

#[test]
fn test_display_implementation() {
    // Test that Display trait is implemented and formats correctly
    let roll = Tile::Roll;
    let empty = Tile::Empty;

    assert_eq!(format!("{}", roll), "@");
    assert_eq!(format!("{}", empty), ".");
}

// Test that the macro works with more variants
#[derive(GridTile)]
enum ComplexTile {
    #[tile('#')]
    Wall,
    #[tile('.')]
    Empty,
    #[tile('@')]
    Player,
    #[tile('O')]
    Box,
}

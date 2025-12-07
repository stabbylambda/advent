// Integration test to verify GridTile can be imported from common crate
// This specifically tests the re-export: pub use common_derive::GridTile;
use common::GridTile;

#[derive(GridTile)]
enum TestTile {
    #[tile('#')]
    Wall,
    #[tile('.')]
    Empty,
    #[tile('@')]
    Player,
}

#[test]
fn test_reexport_compiles() {
    // If this compiles, the re-export is working correctly
    let _wall = TestTile::Wall;
    let _empty = TestTile::Empty;
    let _player = TestTile::Player;
}

#[test]
fn test_debug_via_reexport() {
    // Test that Debug trait is implemented via re-export
    let wall = TestTile::Wall;
    let empty = TestTile::Empty;
    let player = TestTile::Player;

    assert_eq!(format!("{:?}", wall), "#");
    assert_eq!(format!("{:?}", empty), ".");
    assert_eq!(format!("{:?}", player), "@");
}

#[test]
fn test_display_via_reexport() {
    // Test that Display trait is implemented via re-export
    let wall = TestTile::Wall;
    let empty = TestTile::Empty;
    let player = TestTile::Player;

    assert_eq!(format!("{}", wall), "#");
    assert_eq!(format!("{}", empty), ".");
    assert_eq!(format!("{}", player), "@");
}

#[test]
fn test_parser_via_reexport() {
    // Test that the parser method is available via re-export
    use nom::Parser;

    let input = "#";
    let result = TestTile::parser::<_, nom::error::Error<_>>().parse(input);
    assert!(result.is_ok());
    let (remaining, tile) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(format!("{:?}", tile), "#");

    let input = ".";
    let result = TestTile::parser::<_, nom::error::Error<_>>().parse(input);
    assert!(result.is_ok());
    let (remaining, tile) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(format!("{:?}", tile), ".");

    let input = "@";
    let result = TestTile::parser::<_, nom::error::Error<_>>().parse(input);
    assert!(result.is_ok());
    let (remaining, tile) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(format!("{:?}", tile), "@");
}

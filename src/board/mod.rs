/// Generate a [Board][crate::board::board::Board], store its [Tiles][crate::board::tile::Tile], and draw it.
pub mod board;
/// Lots of maths to translate [Coordinates][crate::board::coordinates::Coordinates] to [Tiles][crate::board::tile::Tile].
pub mod coordinates;
/// [Direction][crate::board::direction::Direction] enum.
pub mod direction;
/// [Tile][crate::board::tile::Tile] drawing functions.
pub mod tile;

/// Size of a tile in pixel.
const HEX_SIZE: f64 = 30.0;
/// Window width.
const WIDTH: u32 = 1280;
/// Window heigth.
const HEIGHT: u32 = 600;

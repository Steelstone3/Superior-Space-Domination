use std::ops::Range;

pub const TILE_SIZE: f32 = 1024.0;
pub const STATION_SIZE: f32 = 512.0;
pub const SPACESTATION_DISTANCE_FROM_CENTRE: f32 = 3000.0;
pub const SUN_SIZE: f32 = 1024.0;
pub const PLANET_SIZE: f32 = 400.0;
pub const PLANET_DISTANCE_FROM_SPACESTATION: f32 = 500.0;
pub const MAP_TILES: Range<i32> = -NUMBER_OF_TILES..NUMBER_OF_TILES;
// pub const MAP_SIZE: f32 = TILE_SIZE * NUMBER_OF_TILES as f32;
pub const NUMBER_OF_TILES: i32 = 8;

//! Deterministic world generation.
//!
//! The realm is 160x160 tiles: a river crossed by three stone bridges,
//! four town squares at the compass points, and a neutral plaza at the
//! center-south. Every client grows the same world from the same seed.

pub const MAP_W: u32 = 160;
pub const MAP_H: u32 = 160;

pub const RIVER_TOP: u32 = 57;
pub const RIVER_BOT: u32 = 60;

/// Bridge columns across the river (each 4 tiles wide).
pub const BRIDGES: [u32; 3] = [38, 78, 118];

/// Town square centers: Emberhold NW, Dawnspire NE, Grimforge SW, Tidewatch SE.
pub const TOWN_CENTERS: [(u32, u32); 4] = [(40, 30), (120, 30), (40, 120), (120, 120)];

/// The neutral plaza (x, y, w, h) in tiles.
pub const PLAZA: (u32, u32, u32, u32) = (68, 82, 25, 21);

/// What occupies a tile, before decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Grass,
    Water,
    Bridge,
    Road,
    Square,
    BorderForest,
}

/// Deterministic per-tile hash in [0, 1) — same everywhere, forever.
pub fn tile_hash(x: u32, y: u32, seed: u32) -> f64 {
    let mut h = seed;
    h = (h ^ x).wrapping_mul(0x9E37_79B1);
    h = (h ^ y).wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;
    h as f64 / u32::MAX as f64
}

/// Classify a tile. Decoration (trees, flowers, lanterns) hangs off the
/// same hash so the whole forest is reproducible.
pub fn classify(x: u32, y: u32) -> Tile {
    // border forest ring
    if x < 6 || x >= MAP_W - 6 || y < 8 || y >= MAP_H - 6 {
        return Tile::BorderForest;
    }
    // the river, with its bridges
    if (RIVER_TOP..=RIVER_BOT).contains(&y) {
        for bx in BRIDGES {
            if (bx..bx + 4).contains(&x) {
                return Tile::Bridge;
            }
        }
        return Tile::Water;
    }
    // town squares
    for (cx, cy) in TOWN_CENTERS {
        if x.abs_diff(cx) <= 11 && y.abs_diff(cy) <= 8 {
            return Tile::Square;
        }
    }
    // the plaza
    let (px, py, pw, ph) = PLAZA;
    if (px..px + pw).contains(&x) && (py..py + ph).contains(&y) {
        return Tile::Square;
    }
    // the road grid
    let on_v_road = BRIDGES.iter().any(|bx| (*bx..bx + 4).contains(&x));
    let on_h_road = (30..32).contains(&y) || (91..93).contains(&y) || (120..122).contains(&y);
    if on_v_road || on_h_road {
        return Tile::Road;
    }
    Tile::Grass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_river_is_crossable_exactly_three_ways() {
        let y = RIVER_TOP + 1;
        let mut crossings = 0;
        let mut in_bridge = false;
        for x in 0..MAP_W {
            let bridge = classify(x, y) == Tile::Bridge;
            if bridge && !in_bridge {
                crossings += 1;
            }
            in_bridge = bridge;
        }
        assert_eq!(crossings, 3);
    }

    #[test]
    fn town_centers_are_squares() {
        for (cx, cy) in TOWN_CENTERS {
            assert_eq!(classify(cx, cy), Tile::Square);
        }
    }

    #[test]
    fn the_world_is_deterministic() {
        assert_eq!(tile_hash(42, 87, 11), tile_hash(42, 87, 11));
        assert_ne!(tile_hash(42, 87, 11), tile_hash(87, 42, 11));
    }

    #[test]
    fn the_border_is_forest() {
        assert_eq!(classify(0, 0), Tile::BorderForest);
        assert_eq!(classify(MAP_W - 1, MAP_H - 1), Tile::BorderForest);
    }
}

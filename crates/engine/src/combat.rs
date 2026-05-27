//! Battles: power curves, win probability and loot rolls.
//!
//! A fight is a weighted coin. Power decides the weight; levels move it.

use crate::Nuggets;

/// A hostile the realm can throw at a hunter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enemy {
    pub name: &'static str,
    pub power: u32,
    pub loot_min: Nuggets,
    pub loot_max: Nuggets,
}

/// The roaming goblins, weakest to meanest.
pub const GOBLINS: [Enemy; 4] = [
    Enemy { name: "Goblin Thief", power: 8, loot_min: 4_000, loot_max: 9_000 },
    Enemy { name: "Goblin Maceman", power: 12, loot_min: 6_000, loot_max: 12_000 },
    Enemy { name: "Goblin Archer", power: 16, loot_min: 8_000, loot_max: 16_000 },
    Enemy { name: "Goblin Spearman", power: 22, loot_min: 11_000, loot_max: 22_000 },
];

/// The plaza raid boss. Spawns every 20 minutes, holds for 6.
pub const ORC_CHIEF: Enemy = Enemy {
    name: "Orc Chief",
    power: 42,
    loot_min: 50_000,
    loot_max: 85_000,
};

pub const BOSS_CYCLE_SECS: u64 = 20 * 60;
pub const BOSS_WINDOW_SECS: u64 = 6 * 60;

/// Is the boss on the plaza at this many seconds since the epoch?
pub fn boss_active(unix_secs: u64) -> bool {
    unix_secs % BOSS_CYCLE_SECS < BOSS_WINDOW_SECS
}

/// Probability (0..=1) that `attacker_power` beats `defender_power`.
pub fn win_probability(attacker_power: u32, defender_power: u32) -> f64 {
    let a = attacker_power as f64;
    let d = defender_power as f64;
    if a + d == 0.0 {
        return 0.5;
    }
    a / (a + d)
}

/// A tiny deterministic RNG (SplitMix64) so battles can be replayed.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform integer in [lo, hi].
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }
}

/// Outcome of a resolved battle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleOutcome {
    Victory { loot: Nuggets },
    Defeat,
}

/// Resolve a battle in one roll. UI layers animate on top of this.
pub fn resolve(attacker_power: u32, enemy: &Enemy, rng: &mut Rng) -> BattleOutcome {
    let p = win_probability(attacker_power, enemy.power);
    if rng.next_f64() < p {
        BattleOutcome::Victory {
            loot: rng.range(enemy.loot_min, enemy.loot_max),
        }
    } else {
        BattleOutcome::Defeat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stronger_side_is_favored() {
        assert!(win_probability(40, 8) > 0.8);
        assert!(win_probability(8, 42) < 0.2);
        assert_eq!(win_probability(10, 10), 0.5);
    }

    #[test]
    fn loot_stays_in_range() {
        let mut rng = Rng::new(7);
        for _ in 0..1_000 {
            if let BattleOutcome::Victory { loot } = resolve(100, &ORC_CHIEF, &mut rng) {
                assert!(loot >= ORC_CHIEF.loot_min && loot <= ORC_CHIEF.loot_max);
            }
        }
    }

    #[test]
    fn boss_cycle_opens_and_closes() {
        assert!(boss_active(0));
        assert!(boss_active(BOSS_WINDOW_SECS - 1));
        assert!(!boss_active(BOSS_WINDOW_SECS));
        assert!(boss_active(BOSS_CYCLE_SECS));
    }

    #[test]
    fn replays_are_deterministic() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }
}

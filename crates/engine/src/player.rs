//! Heroes: experience, levels and the power curve.

use crate::Nuggets;

pub const XP_PER_WIN: u64 = 12;
pub const XP_PER_LOSS: u64 = 4;
pub const STARTING_PURSE: Nuggets = 20_000;

/// XP required to clear a given level.
pub fn xp_for_level(level: u32) -> u64 {
    40 + level as u64 * 35
}

/// Power at a given level. Every level is +3 power, forever.
pub fn power_for_level(level: u32) -> u32 {
    10 + level * 3
}

/// A hunter in the realm.
#[derive(Debug, Clone)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub xp: u64,
    pub purse: Nuggets,
    pub wins: u32,
    pub losses: u32,
}

impl Hero {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: 1,
            xp: 0,
            purse: STARTING_PURSE,
            wins: 0,
            losses: 0,
        }
    }

    pub fn power(&self) -> u32 {
        power_for_level(self.level)
    }

    /// Grant XP, cascading level-ups. Returns how many levels were gained.
    pub fn gain_xp(&mut self, amount: u64) -> u32 {
        self.xp += amount;
        let mut gained = 0;
        while self.xp >= xp_for_level(self.level) {
            self.xp -= xp_for_level(self.level);
            self.level += 1;
            gained += 1;
        }
        gained
    }

    /// Record a battle result. Even defeat teaches.
    pub fn record_battle(&mut self, won: bool) -> u32 {
        if won {
            self.wins += 1;
            self.gain_xp(XP_PER_WIN)
        } else {
            self.losses += 1;
            self.gain_xp(XP_PER_LOSS)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_cascade() {
        let mut hero = Hero::new("Wanderer");
        let gained = hero.gain_xp(1_000);
        assert!(gained >= 2);
        assert_eq!(hero.level, 1 + gained);
    }

    #[test]
    fn power_compounds_with_level() {
        assert_eq!(power_for_level(1), 13);
        assert_eq!(power_for_level(10), 40);
        assert_eq!(power_for_level(20), 70);
    }

    #[test]
    fn defeat_still_teaches() {
        let mut hero = Hero::new("Wanderer");
        hero.record_battle(false);
        assert_eq!(hero.xp, XP_PER_LOSS);
        assert_eq!(hero.losses, 1);
    }
}

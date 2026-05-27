//! The four dominions: buffs, treasuries and the race for the crown.

use crate::Nuggets;

/// The four AI-governed dominions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum District {
    /// Emberhold — governed by Claude. Careful, thorough, audited twice.
    Emberhold,
    /// Dawnspire — governed by ChatGPT. Eleven plans, all of them at once.
    Dawnspire,
    /// Grimforge — governed by Grok. Founded on a dare.
    Grimforge,
    /// Tidewatch — governed by Gemini. Two regents, one voice.
    Tidewatch,
}

pub const ALL_DISTRICTS: [District; 4] = [
    District::Emberhold,
    District::Dawnspire,
    District::Grimforge,
    District::Tidewatch,
];

/// Citizenship buffs, in basis points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Buff {
    pub battle_power_bps: u64,
    pub stake_yield_bps: u64,
    pub battle_loot_bps: u64,
}

impl District {
    pub fn name(self) -> &'static str {
        match self {
            District::Emberhold => "Emberhold",
            District::Dawnspire => "Dawnspire",
            District::Grimforge => "Grimforge",
            District::Tidewatch => "Tidewatch",
        }
    }

    pub fn governed_by(self) -> &'static str {
        match self {
            District::Emberhold => "Claude",
            District::Dawnspire => "ChatGPT",
            District::Grimforge => "Grok",
            District::Tidewatch => "Gemini",
        }
    }

    /// What citizenship earns you.
    pub fn buff(self) -> Buff {
        match self {
            District::Emberhold => Buff { battle_power_bps: 0, stake_yield_bps: 1_000, battle_loot_bps: 0 },
            District::Dawnspire => Buff { battle_power_bps: 500, stake_yield_bps: 500, battle_loot_bps: 0 },
            District::Grimforge => Buff { battle_power_bps: 1_000, stake_yield_bps: 0, battle_loot_bps: 0 },
            District::Tidewatch => Buff { battle_power_bps: 0, stake_yield_bps: 0, battle_loot_bps: 800 },
        }
    }
}

/// A town's living ledger.
#[derive(Debug, Clone, Copy, Default)]
pub struct Town {
    pub power: Nuggets,
    pub treasury: Nuggets,
    pub citizens: u32,
}

impl Town {
    /// Staked gold counts 1:1 toward power.
    pub fn stake(&mut self, amount: Nuggets) {
        self.power += amount;
    }

    /// Unstaking withdraws the same power it added.
    pub fn unstake(&mut self, amount: Nuggets) {
        self.power = self.power.saturating_sub(amount);
    }

    /// Apply a loot settlement (tax or delegation) to the town.
    pub fn absorb(&mut self, settlement: crate::economy::LootSettlement) {
        self.treasury += settlement.to_treasury;
        self.power += settlement.town_power_delta;
    }
}

/// Which dominion currently wears the crown.
pub fn crown_holder(towns: &[(District, Town)]) -> Option<District> {
    towns
        .iter()
        .max_by_key(|(_, t)| t.power)
        .map(|(d, _)| *d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::{settle_loot, LootChoice};

    #[test]
    fn delegation_outpaces_taxed_keeping() {
        let mut kept = Town::default();
        let mut pledged = Town::default();
        kept.absorb(settle_loot(10_000, LootChoice::Keep));
        pledged.absorb(settle_loot(10_000, LootChoice::Delegate));
        assert!(pledged.power > kept.power * 5);
    }

    #[test]
    fn the_crown_follows_power() {
        let towns = vec![
            (District::Emberhold, Town { power: 4_210_000, ..Default::default() }),
            (District::Dawnspire, Town { power: 4_890_000, ..Default::default() }),
            (District::Grimforge, Town { power: 3_660_000, ..Default::default() }),
            (District::Tidewatch, Town { power: 3_975_000, ..Default::default() }),
        ];
        assert_eq!(crown_holder(&towns), Some(District::Dawnspire));
    }

    #[test]
    fn every_district_has_a_mind() {
        for d in ALL_DISTRICTS {
            assert!(!d.governed_by().is_empty());
        }
    }
}

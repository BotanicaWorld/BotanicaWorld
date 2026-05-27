//! The gold loop: crown tax, delegation, staking yield and burns.
//!
//! Every purse a hunter keeps pays the crown its fifth. Every purse
//! delegated skips the tax and counts double toward town power. Every
//! coin spent in a shop or lost in a wager leaves the realm forever.

use crate::Nuggets;

/// Crown tax on kept battle loot, in basis points (20.00%).
pub const CROWN_TAX_BPS: u64 = 2_000;

/// Passive staking yield per minute, in basis points of the staked amount (0.4%/min).
pub const STAKE_YIELD_PER_MIN_BPS: u64 = 40;

/// Minimum single stake.
pub const MIN_STAKE: Nuggets = 5_000;

/// Gold dropped into the void when a hunter flees a lost battle.
pub const DEFEAT_PENALTY: Nuggets = 2_500;

/// What a hunter chooses to do with a fresh purse of loot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LootChoice {
    /// Keep it — the crown taxes it on the way into your pocket.
    Keep,
    /// Hand the whole purse to your town: no tax, double power, governance.
    Delegate,
}

/// The fully-resolved consequences of a loot decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LootSettlement {
    pub to_player: Nuggets,
    pub to_treasury: Nuggets,
    pub town_power_delta: Nuggets,
    pub governance_points: Nuggets,
}

/// Settle a purse of loot according to the hunter's choice.
pub fn settle_loot(amount: Nuggets, choice: LootChoice) -> LootSettlement {
    match choice {
        LootChoice::Keep => {
            let tax = amount * CROWN_TAX_BPS / 10_000;
            LootSettlement {
                to_player: amount - tax,
                to_treasury: tax,
                town_power_delta: tax,
                governance_points: 0,
            }
        }
        LootChoice::Delegate => LootSettlement {
            to_player: 0,
            to_treasury: amount,
            town_power_delta: amount * 2,
            governance_points: amount,
        },
    }
}

/// Yield earned on a stake over a duration, floor-rounded.
pub fn staking_yield(staked: Nuggets, minutes: u64) -> Nuggets {
    staked * STAKE_YIELD_PER_MIN_BPS * minutes / 10_000
}

/// A single irreversible exit of gold from the realm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurnReason {
    ShopPurchase,
    LostWager,
    DefeatPenalty,
}

/// Append-only record of everything the realm has burned.
#[derive(Debug, Default)]
pub struct BurnLedger {
    entries: Vec<(BurnReason, Nuggets)>,
}

impl BurnLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn burn(&mut self, reason: BurnReason, amount: Nuggets) {
        self.entries.push((reason, amount));
    }

    /// Gold gone forever.
    pub fn total_burned(&self) -> Nuggets {
        self.entries.iter().map(|(_, n)| n).sum()
    }

    /// Supply still walking the realm.
    pub fn circulating(&self) -> Nuggets {
        crate::TOTAL_SUPPLY.saturating_sub(self.total_burned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeping_pays_the_crown_its_fifth() {
        let s = settle_loot(10_000, LootChoice::Keep);
        assert_eq!(s.to_player, 8_000);
        assert_eq!(s.to_treasury, 2_000);
        assert_eq!(s.town_power_delta, 2_000);
        assert_eq!(s.governance_points, 0);
    }

    #[test]
    fn delegation_doubles_power_and_earns_governance() {
        let s = settle_loot(10_000, LootChoice::Delegate);
        assert_eq!(s.to_player, 0);
        assert_eq!(s.to_treasury, 10_000);
        assert_eq!(s.town_power_delta, 20_000);
        assert_eq!(s.governance_points, 10_000);
    }

    #[test]
    fn an_hour_of_yield_is_24_percent() {
        assert_eq!(staking_yield(100_000, 60), 24_000);
    }

    #[test]
    fn burns_only_shrink_supply() {
        let mut ledger = BurnLedger::new();
        ledger.burn(BurnReason::ShopPurchase, 8_000);
        ledger.burn(BurnReason::LostWager, 5_000);
        assert_eq!(ledger.total_burned(), 13_000);
        assert_eq!(ledger.circulating(), crate::TOTAL_SUPPLY - 13_000);
    }
}

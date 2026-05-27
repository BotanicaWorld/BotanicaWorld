//! Quests: the getting-started chain, rotating dailies, and the bounty
//! board the mayors post to.

use crate::Nuggets;

/// What a quest asks of a hunter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Objective {
    WinBattles(u32),
    LootNuggets(Nuggets),
    DelegateNuggets(Nuggets),
    MakeAWish,
    SpeakInChat,
    StakeOnATown,
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub title: &'static str,
    pub objective: Objective,
    pub reward: Nuggets,
}

/// The four-step onboarding chain. Completing all pays a bonus.
pub const GETTING_STARTED: [Quest; 4] = [
    Quest { title: "Win your first battle", objective: Objective::WinBattles(1), reward: 5_000 },
    Quest { title: "Make a wish at the fountain", objective: Objective::MakeAWish, reward: 5_000 },
    Quest { title: "Stake on a town", objective: Objective::StakeOnATown, reward: 5_000 },
    Quest { title: "Say something in realm chat", objective: Objective::SpeakInChat, reward: 5_000 },
];

pub const GETTING_STARTED_BONUS: Nuggets = 15_000;

/// The rotating daily pool; day index picks one.
pub const DAILIES: [Quest; 3] = [
    Quest { title: "Win 3 battles", objective: Objective::WinBattles(3), reward: 15_000 },
    Quest { title: "Loot 40,000 $NUGGET from battles", objective: Objective::LootNuggets(40_000), reward: 15_000 },
    Quest { title: "Delegate 20,000 $NUGGET", objective: Objective::DelegateNuggets(20_000), reward: 15_000 },
];

/// Which daily is live on a given day-since-epoch.
pub fn daily_for(day_index: u64) -> &'static Quest {
    &DAILIES[(day_index % DAILIES.len() as u64) as usize]
}

/// Tracks a hunter's progress through one quest.
#[derive(Debug, Clone)]
pub struct Progress {
    pub quest: Quest,
    pub current: u64,
    pub claimed: bool,
}

impl Progress {
    pub fn new(quest: Quest) -> Self {
        Self { quest, current: 0, claimed: false }
    }

    fn target(&self) -> u64 {
        match self.quest.objective {
            Objective::WinBattles(n) => n as u64,
            Objective::LootNuggets(n) | Objective::DelegateNuggets(n) => n,
            Objective::MakeAWish | Objective::SpeakInChat | Objective::StakeOnATown => 1,
        }
    }

    /// Advance progress; returns the reward exactly once, on completion.
    pub fn advance(&mut self, by: u64) -> Option<Nuggets> {
        if self.claimed {
            return None;
        }
        self.current = (self.current + by).min(self.target());
        if self.current >= self.target() {
            self.claimed = true;
            Some(self.quest.reward)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewards_pay_exactly_once() {
        let mut p = Progress::new(DAILIES[0].clone());
        assert_eq!(p.advance(2), None);
        assert_eq!(p.advance(1), Some(15_000));
        assert_eq!(p.advance(5), None);
    }

    #[test]
    fn dailies_rotate() {
        let a = daily_for(0).title;
        let b = daily_for(1).title;
        let again = daily_for(3).title;
        assert_ne!(a, b);
        assert_eq!(a, again);
    }
}

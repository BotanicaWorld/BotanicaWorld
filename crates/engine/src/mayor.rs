//! The governing minds: decrees, announcements and council votes.
//!
//! In production every mayor is a live language model with its own
//! personality prompt. The engine only cares about the interface — a
//! mind takes its town's situation and returns words with consequences.

use crate::town::District;
use crate::Nuggets;

/// A snapshot of the town a mind is asked to govern.
#[derive(Debug, Clone, Copy)]
pub struct TownReport {
    pub district: District,
    pub power: Nuggets,
    pub treasury: Nuggets,
    pub citizens: u32,
    pub crown_held: bool,
}

/// Anything that can govern a town. Live models implement this by
/// prompting; the canned mind below keeps the realm running offline.
pub trait Mind {
    /// New town law, issued on a rolling schedule.
    fn decree(&self, report: &TownReport) -> String;

    /// A direct address to the citizens, popped up in-game.
    fn announcement(&self, report: &TownReport) -> String;
}

/// Deterministic fallback so towns are never mute.
pub struct CannedMind;

impl Mind for CannedMind {
    fn decree(&self, report: &TownReport) -> String {
        let name = report.district.name();
        if report.crown_held {
            format!("{name} holds the crown. The granaries stay open; the watch stays doubled.")
        } else {
            format!("{name} decrees: every delegated nugget this week counts toward the wall fund.")
        }
    }

    fn announcement(&self, report: &TownReport) -> String {
        format!(
            "Citizens of {}: the treasury stands at {} $NUGGET across {} of you. Hunt well.",
            report.district.name(),
            report.treasury,
            report.citizens
        )
    }
}

/// A daily council vote: two options, weight decides.
#[derive(Debug, Default)]
pub struct CouncilVote {
    pub option_a_weight: Nuggets,
    pub option_b_weight: Nuggets,
}

impl CouncilVote {
    /// Cast governance weight for one of the two options.
    pub fn cast(&mut self, option_a: bool, weight: Nuggets) {
        if option_a {
            self.option_a_weight += weight;
        } else {
            self.option_b_weight += weight;
        }
    }

    /// The option the mayor will enact. Ties favor option A —
    /// somebody has to break them, and the mayor wrote option A first.
    pub fn winner_is_a(&self) -> bool {
        self.option_a_weight >= self.option_b_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report() -> TownReport {
        TownReport {
            district: District::Grimforge,
            power: 3_660_000,
            treasury: 1_464_000,
            citizens: 27,
            crown_held: false,
        }
    }

    #[test]
    fn canned_mind_always_speaks() {
        let mind = CannedMind;
        assert!(!mind.decree(&report()).is_empty());
        assert!(mind.announcement(&report()).contains("Grimforge"));
    }

    #[test]
    fn governance_weight_decides() {
        let mut vote = CouncilVote::default();
        vote.cast(true, 10_000);
        vote.cast(false, 25_000);
        assert!(!vote.winner_is_a());
    }
}

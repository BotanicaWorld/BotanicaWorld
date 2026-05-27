//! Duels are consensual: a request, an answer, then a fight for the pot.

use crate::combat::{resolve, BattleOutcome, Enemy, Rng};
use crate::Nuggets;

/// Standard duel wager. Winner takes both stakes.
pub const WAGER: Nuggets = 5_000;

/// The life of a duel, from challenge to settlement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Duel {
    /// Challenge sent; the challenged hunter must answer.
    Requested { challenger: String, challenged: String, wager: Nuggets },
    /// Declined. No gold moves.
    Declined,
    /// Accepted and fought. The pot has a home.
    Settled { winner: String, pot: Nuggets },
}

impl Duel {
    pub fn request(challenger: impl Into<String>, challenged: impl Into<String>) -> Self {
        Duel::Requested {
            challenger: challenger.into(),
            challenged: challenged.into(),
            wager: WAGER,
        }
    }

    /// The challenged hunter answers. Accepting fights immediately.
    pub fn answer(self, accept: bool, challenger_power: u32, challenged_power: u32, rng: &mut Rng) -> Duel {
        match self {
            Duel::Requested { challenger, challenged, wager } => {
                if !accept {
                    return Duel::Declined;
                }
                // Fight: the challenged hunter is modelled as the defender.
                let defender = Enemy {
                    name: "duelist",
                    power: challenged_power,
                    loot_min: wager * 2,
                    loot_max: wager * 2,
                };
                let winner = match resolve(challenger_power, &defender, rng) {
                    BattleOutcome::Victory { .. } => challenger,
                    BattleOutcome::Defeat => challenged,
                };
                Duel::Settled { winner, pot: wager * 2 }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declining_moves_no_gold() {
        let duel = Duel::request("A", "B").answer(false, 50, 10, &mut Rng::new(1));
        assert_eq!(duel, Duel::Declined);
    }

    #[test]
    fn accepting_settles_the_pot() {
        let duel = Duel::request("A", "B").answer(true, 50, 10, &mut Rng::new(1));
        match duel {
            Duel::Settled { pot, .. } => assert_eq!(pot, WAGER * 2),
            other => panic!("expected settlement, got {other:?}"),
        }
    }
}

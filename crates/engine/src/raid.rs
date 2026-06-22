//! Co-op raids — the coming season.
//!
//! The types below are the contract the raid season will ship against:
//! a four-hunter party, a formation, and a shared pot split by damage.
//! The in-game raid screen already shows this shape, greyed out.

use crate::Nuggets;

/// Maximum hunters in one raid party, leader included.
pub const PARTY_SIZE: usize = 4;

/// How a party lines up against the boss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formation {
    /// Front-line brawl: highest damage, highest risk.
    Vanguard,
    /// Slow and heavy: damage reduction for the whole party.
    Siege,
    /// Fast raids: shorter fights, smaller pots, more of them.
    Skirmish,
}

/// A raid party being assembled.
#[derive(Debug, Clone)]
pub struct Party {
    pub leader: String,
    pub members: Vec<String>,
    pub formation: Formation,
}

/// Why a hunter could not join.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinError {
    PartyFull,
    AlreadyIn,
}

impl Party {
    pub fn new(leader: impl Into<String>) -> Self {
        Self {
            leader: leader.into(),
            members: Vec::new(),
            formation: Formation::Vanguard,
        }
    }

    pub fn size(&self) -> usize {
        1 + self.members.len()
    }

    pub fn invite(&mut self, hunter: impl Into<String>) -> Result<(), JoinError> {
        let hunter = hunter.into();
        if self.size() >= PARTY_SIZE {
            return Err(JoinError::PartyFull);
        }
        if hunter == self.leader || self.members.contains(&hunter) {
            return Err(JoinError::AlreadyIn);
        }
        self.members.push(hunter);
        Ok(())
    }
}

/// Split a raid pot by damage dealt, remainder to the leader.
/// Order of `damage` matches leader-first party order.
pub fn split_pot(pot: Nuggets, damage: &[u64]) -> Vec<Nuggets> {
    let total: u64 = damage.iter().sum();
    if total == 0 {
        let mut shares = vec![0; damage.len()];
        if let Some(first) = shares.first_mut() {
            *first = pot;
        }
        return shares;
    }
    let mut shares: Vec<Nuggets> = damage.iter().map(|d| pot * d / total).collect();
    let distributed: Nuggets = shares.iter().sum();
    if let Some(first) = shares.first_mut() {
        *first += pot - distributed;
    }
    shares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parties_cap_at_four() {
        let mut party = Party::new("Leader");
        assert!(party.invite("A").is_ok());
        assert!(party.invite("B").is_ok());
        assert!(party.invite("C").is_ok());
        assert_eq!(party.invite("D"), Err(JoinError::PartyFull));
    }

    #[test]
    fn no_double_joining() {
        let mut party = Party::new("Leader");
        party.invite("A").unwrap();
        assert_eq!(party.invite("A"), Err(JoinError::AlreadyIn));
        assert_eq!(party.invite("Leader"), Err(JoinError::AlreadyIn));
    }

    #[test]
    fn the_pot_splits_by_damage_and_loses_nothing() {
        let shares = split_pot(85_000, &[50, 30, 20, 0]);
        assert_eq!(shares.iter().sum::<u64>(), 85_000);
        assert!(shares[0] > shares[1]);
        assert_eq!(shares[3], 0);
    }
}

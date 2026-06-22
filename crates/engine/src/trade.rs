//! Player-to-player trades: an offer, a judgement, a settlement.
//!
//! Nobody trades at a loss on purpose. Counterparties accept offers
//! that profit them — and occasionally take a generous whim.

use crate::combat::Rng;
use crate::Nuggets;

/// A proposed exchange of gold, from the offerer's perspective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offer {
    pub give: Nuggets,
    pub receive: Nuggets,
}

/// The counterparty's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Accepted,
    Declined,
}

/// Threshold at which an offer is clearly profitable to accept:
/// asking for no more than 90% of what is given.
const CLEAR_PROFIT_BPS: u64 = 9_000;

/// Slightly-unfavorable offers (up to 120%) are taken on a whim,
/// about a third of the time. Generosity greases the realm.
const WHIM_LIMIT_BPS: u64 = 12_000;
const WHIM_CHANCE: f64 = 0.35;

/// How a rational-but-mortal counterparty judges an offer.
pub fn judge(offer: Offer, rng: &mut Rng) -> Answer {
    if offer.receive * 10_000 <= offer.give * CLEAR_PROFIT_BPS {
        return Answer::Accepted;
    }
    if offer.receive * 10_000 <= offer.give * WHIM_LIMIT_BPS && rng.next_f64() < WHIM_CHANCE {
        return Answer::Accepted;
    }
    Answer::Declined
}

/// Net change to the offerer's purse if the trade settles.
pub fn settle(offer: Offer) -> i64 {
    offer.receive as i64 - offer.give as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profitable_offers_are_always_taken() {
        let mut rng = Rng::new(3);
        let offer = Offer { give: 10_000, receive: 5_000 };
        for _ in 0..50 {
            assert_eq!(judge(offer, &mut rng), Answer::Accepted);
        }
    }

    #[test]
    fn robbery_is_always_declined() {
        let mut rng = Rng::new(3);
        let offer = Offer { give: 1_000, receive: 50_000 };
        for _ in 0..50 {
            assert_eq!(judge(offer, &mut rng), Answer::Declined);
        }
    }

    #[test]
    fn settlement_is_signed() {
        assert_eq!(settle(Offer { give: 3_000, receive: 8_000 }), 5_000);
        assert_eq!(settle(Offer { give: 8_000, receive: 3_000 }), -5_000);
    }
}

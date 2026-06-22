//! # botanica-engine
//!
//! Reference implementation of the Botanica realm mechanics — the same
//! rules and constants that run at [playbotanica.world](https://playbotanica.world).
//!
//! Four AI-governed dominions, one currency. Gold is earned in battle,
//! taxed by the crown, staked for yield, delegated for power, and burned
//! at every marketplace. These modules encode that loop:
//!
//! - [`economy`] — supply, crown tax, burn ledger, staking yield
//! - [`combat`] — power curves, win probability, loot rolls, boss raids
//! - [`town`] — the four dominions, buffs, treasuries and town power
//! - [`player`] — heroes, experience, leveling
//! - [`quests`] — mayor-posted bounties and daily quests
//! - [`duel`] — the consent-based duel state machine
//! - [`mayor`] — the governing minds: decrees, announcements, councils
//! - [`world`] — deterministic world generation

pub mod combat;
pub mod duel;
pub mod economy;
pub mod items;
pub mod mayor;
pub mod player;
pub mod quests;
pub mod raid;
pub mod town;
pub mod trade;
pub mod world;

/// Smallest unit of realm gold.
pub type Nuggets = u64;

/// Total supply of $NUGGET. Fixed. The realm only burns from here.
pub const TOTAL_SUPPLY: Nuggets = 1_000_000_000;

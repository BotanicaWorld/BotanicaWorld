//! A day in the realm, simulated.
//!
//! ```sh
//! cargo run --example realm_day
//! ```

use botanica_engine::combat::{resolve, BattleOutcome, Rng, GOBLINS, ORC_CHIEF};
use botanica_engine::economy::{settle_loot, staking_yield, BurnLedger, BurnReason, LootChoice};
use botanica_engine::player::Hero;
use botanica_engine::town::{crown_holder, District, Town, ALL_DISTRICTS};

fn main() {
    let mut rng = Rng::new(20260703);
    let mut hero = Hero::new("Wanderer");
    let mut burns = BurnLedger::new();
    let mut towns: Vec<(District, Town)> = ALL_DISTRICTS
        .iter()
        .map(|d| (*d, Town { power: 4_000_000, treasury: 1_600_000, citizens: 25 }))
        .collect();
    let home = 0; // Emberhold

    println!("== dawn: {} enters the realm with {} $NUGGET ==\n", hero.name, hero.purse);

    // A morning of goblin hunting.
    for _ in 0..12 {
        let target = &GOBLINS[(rng.next_u64() % 4) as usize];
        match resolve(hero.power(), target, &mut rng) {
            BattleOutcome::Victory { loot } => {
                hero.record_battle(true);
                // keep some, delegate some
                let choice = if loot > 15_000 { LootChoice::Delegate } else { LootChoice::Keep };
                let s = settle_loot(loot, choice);
                hero.purse += s.to_player;
                towns[home].1.absorb(s);
                println!("victory over {:<16} loot {:>6}  ({:?})", target.name, loot, choice);
            }
            BattleOutcome::Defeat => {
                hero.record_battle(false);
                let penalty = 2_500.min(hero.purse);
                hero.purse -= penalty;
                burns.burn(BurnReason::DefeatPenalty, penalty);
                println!("defeated by {:<16} dropped {:>5} fleeing", target.name, penalty);
            }
        }
    }

    // The boss window opens.
    println!("\n== the {} storms the plaza ==", ORC_CHIEF.name);
    match resolve(hero.power(), &ORC_CHIEF, &mut rng) {
        BattleOutcome::Victory { loot } => {
            hero.record_battle(true);
            let s = settle_loot(loot, LootChoice::Keep);
            hero.purse += s.to_player;
            towns[home].1.absorb(s);
            println!("the chief falls. bounty {loot}, kept {} after tax", s.to_player);
        }
        BattleOutcome::Defeat => {
            hero.record_battle(false);
            println!("the chief stands. tomorrow, then.");
        }
    }

    // Overnight: stake half the purse.
    let stake = hero.purse / 2;
    towns[home].1.stake(stake);
    let overnight = staking_yield(stake, 8 * 60);
    hero.purse = hero.purse - stake + stake + overnight;
    towns[home].1.unstake(stake);

    println!("\n== dusk ==");
    println!("level {} ({}W {}L), purse {} $NUGGET", hero.level, hero.wins, hero.losses, hero.purse);
    println!("overnight stake of {stake} yielded {overnight}");
    println!("burned today: {} (circulating: {})", burns.total_burned(), burns.circulating());
    if let Some(crowned) = crown_holder(&towns) {
        println!("the crown rests on {}", crowned.name());
    }
}

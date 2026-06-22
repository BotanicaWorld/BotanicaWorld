//! The general store and the hero's armoury.
//!
//! Everything bought here is burned — shop gold leaves the realm.

use crate::Nuggets;

/// What the general store sells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    /// Heal 40 HP mid-battle. Consumed on use.
    BattlePotion,
    /// +30 XP, applied instantly.
    TrainingScroll,
    /// Walk the realm as the menace itself.
    GoblinThiefSkin,
    /// Big mace. Bigger attitude.
    GoblinMacemanSkin,
}

impl Item {
    pub fn price(self) -> Nuggets {
        match self {
            Item::BattlePotion => 8_000,
            Item::TrainingScroll => 15_000,
            Item::GoblinThiefSkin | Item::GoblinMacemanSkin => 40_000,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Item::BattlePotion => "Battle Potion",
            Item::TrainingScroll => "Scroll of Training",
            Item::GoblinThiefSkin => "Goblin Thief Skin",
            Item::GoblinMacemanSkin => "Goblin Maceman Skin",
        }
    }
}

/// HP restored by one potion.
pub const POTION_HEAL: u32 = 40;

/// XP granted by one training scroll.
pub const SCROLL_XP: u64 = 30;

/// The six slots on every hero sheet. Reserved today, filled in a
/// future season — armour drops are on the roadmap, not in the shop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmourSlot {
    Helmet,
    Chest,
    Legs,
    Boots,
    Weapon,
    Trinket,
}

pub const ARMOUR_SLOTS: [ArmourSlot; 6] = [
    ArmourSlot::Helmet,
    ArmourSlot::Chest,
    ArmourSlot::Legs,
    ArmourSlot::Boots,
    ArmourSlot::Weapon,
    ArmourSlot::Trinket,
];

/// A hero's carried goods.
#[derive(Debug, Default, Clone)]
pub struct Inventory {
    pub potions: u32,
    pub skins: Vec<Item>,
}

/// Errors the shopkeeper can hand you instead of goods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopError {
    NotEnoughGold,
    AlreadyOwned,
}

/// Buy an item: deducts (burns) the price, updates the inventory.
/// Returns the new purse on success.
pub fn buy(purse: Nuggets, item: Item, inv: &mut Inventory) -> Result<Nuggets, ShopError> {
    let price = item.price();
    if purse < price {
        return Err(ShopError::NotEnoughGold);
    }
    match item {
        Item::BattlePotion => inv.potions += 1,
        Item::TrainingScroll => {} // XP applied by the caller
        skin => {
            if inv.skins.contains(&skin) {
                return Err(ShopError::AlreadyOwned);
            }
            inv.skins.push(skin);
        }
    }
    Ok(purse - price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buying_burns_the_price() {
        let mut inv = Inventory::default();
        let purse = buy(20_000, Item::BattlePotion, &mut inv).unwrap();
        assert_eq!(purse, 12_000);
        assert_eq!(inv.potions, 1);
    }

    #[test]
    fn skins_cannot_be_bought_twice() {
        let mut inv = Inventory::default();
        let purse = buy(100_000, Item::GoblinThiefSkin, &mut inv).unwrap();
        assert_eq!(buy(purse, Item::GoblinThiefSkin, &mut inv), Err(ShopError::AlreadyOwned));
    }

    #[test]
    fn the_shop_refuses_the_poor() {
        let mut inv = Inventory::default();
        assert_eq!(buy(1_000, Item::TrainingScroll, &mut inv), Err(ShopError::NotEnoughGold));
    }

    #[test]
    fn six_slots_wait_for_their_season() {
        assert_eq!(ARMOUR_SLOTS.len(), 6);
    }
}

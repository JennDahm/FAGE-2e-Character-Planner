#![allow(dead_code)]

use strum::{EnumIter, IntoEnumIterator};

use super::ability::{Ability, Focus};
use super::numbers::{Dice, DiceWithMod};

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum WeaponGroup {
    Axes,
    BlackPowder,
    Bludgeons,
    Bows,
    Brawling,
    Dueling,
    HeavyBlades,
    Lances,
    LightBlades,
    Polearms,
    Slings,
    Spears,
    Staves,
}

impl WeaponGroup {
    pub fn weapons(self) -> impl Iterator<Item = Weapon> {
        Weapon::iter().filter(move |w| w.group() == self)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Axes => "Axes",
            Self::BlackPowder => "Black Powder",
            Self::Bludgeons => "Bludgeons",
            Self::Bows => "Bows",
            Self::Brawling => "Brawling",
            Self::Dueling => "Dueling",
            Self::HeavyBlades => "Heavy Blades",
            Self::Lances => "Lances",
            Self::LightBlades => "Light Blades",
            Self::Polearms => "Polearms",
            Self::Slings => "Slings",
            Self::Spears => "Spears",
            Self::Staves => "Staves",
        }
    }

    pub fn focus(&self) -> Focus {
        match self {
            Self::Axes => Focus::FightingAxes,
            Self::BlackPowder => Focus::AccuracyBlackPowder,
            Self::Bludgeons => Focus::FightingBludgeons,
            Self::Bows => Focus::AccuracyBows,
            Self::Brawling => Focus::AccuracyBrawling,
            Self::Dueling => Focus::AccuracyDueling,
            Self::HeavyBlades => Focus::FightingHeavyBlades,
            Self::Lances => Focus::FightingLances,
            Self::LightBlades => Focus::AccuracyLightBlades,
            Self::Polearms => Focus::FightingPolearms,
            Self::Slings => Focus::AccuracySlings,
            Self::Spears => Focus::FightingSpears,
            Self::Staves => Focus::AccuracyStaves,
        }
    }

    pub fn attack_ability(&self) -> Ability {
        self.focus().ability()
    }

    pub fn damage_ability(&self) -> Ability {
        match self.attack_ability() {
            Ability::Accuracy => Ability::Perception,
            Ability::Fighting => Ability::Strength,
            _ => panic!("Unsupported attack ability type!"),
        }
    }
}

impl std::fmt::Display for WeaponGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Weapon {
    // AXES
    BattleAxe,
    ThrowingAxe,
    TwoHandedAxe,

    // BLACK POWDER
    Arquebus,
    Blunderbuss,
    Musket,
    Pistol,

    // BLUDGEONS
    Mace,
    Maul,
    TwoHandedMaul,

    // BOWS
    Crossbow,
    ShortBow,
    LongBow,

    // BRAWLING
    Fist,
    Gauntlet,
    ImprovisedWeapon,

    // DUELING
    MainGauche,
    Rapier,
    SpikedBuckler, // TODO: Handle +1 defense vs melee-only?

    // HEAVY BLADES
    BastardSword,
    LongSword,
    TwoHandedSword,

    // LANCES
    HeavyLance,
    JoustingLance,
    LightLance,

    // LIGHT BLADES
    Dagger,
    ShortSword,
    ThrowingKnife,

    // POLEARMS
    Glaive,
    Halberd,
    MilitaryFork,

    // SLINGS
    Fustibale,
    HuntingSling,
    Slingshot,

    // SPEARS
    Spear,
    ThrowingSpear,
    TwoHandedSpear,

    // STAVES
    Club,
    Morningstar,
    Quarterstaff,
}

impl Weapon {
    pub fn group(&self) -> WeaponGroup {
        match self {
            // AXES
            Self::BattleAxe |
            Self::ThrowingAxe |
            Self::TwoHandedAxe => WeaponGroup::Axes,

            // BLACK POWDER
            Self::Arquebus |
            Self::Blunderbuss |
            Self::Musket |
            Self::Pistol => WeaponGroup::BlackPowder,

            // BLUDGEONS
            Self::Mace |
            Self::Maul |
            Self::TwoHandedMaul => WeaponGroup::Bludgeons,

            // BOWS
            Self::Crossbow |
            Self::ShortBow |
            Self::LongBow => WeaponGroup::Bows,

            // BRAWLING
            Self::Fist |
            Self::Gauntlet |
            Self::ImprovisedWeapon => WeaponGroup::Brawling,

            // DUELING
            Self::MainGauche |
            Self::Rapier |
            Self::SpikedBuckler => WeaponGroup::Dueling,

            // HEAVY BLADES
            Self::BastardSword |
            Self::LongSword |
            Self::TwoHandedSword => WeaponGroup::HeavyBlades,

            // LANCES
            Self::HeavyLance |
            Self::JoustingLance |
            Self::LightLance => WeaponGroup::Lances,

            // LIGHT BLADES
            Self::Dagger |
            Self::ShortSword |
            Self::ThrowingKnife => WeaponGroup::LightBlades,

            // POLEARMS
            Self::Glaive |
            Self::Halberd |
            Self::MilitaryFork => WeaponGroup::Polearms,

            // SLINGS
            Self::Fustibale |
            Self::HuntingSling |
            Self::Slingshot => WeaponGroup::Slings,

            // SPEARS
            Self::Spear |
            Self::ThrowingSpear |
            Self::TwoHandedSpear => WeaponGroup::Spears,

            // STAVES
            Self::Club |
            Self::Morningstar |
            Self::Quarterstaff => WeaponGroup::Staves,
        }
    }

    pub fn properties(&self) -> WeaponProperties {
        match self {
            // AXES
            Self::BattleAxe => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 0 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },
            Self::ThrowingAxe => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 2 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 4,
                    long_range_yards: Some(8),
                    reload_is_major_action: false,
                }),
            },
            Self::TwoHandedAxe => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(3), modifier: 0 },
                min_strength: Some(3),
                two_handed: true,
                missile_properties: None,
            },

            // BLACK POWDER
            Self::Arquebus => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 3 },
                min_strength: None,
                two_handed: true,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 12,
                    long_range_yards: Some(24),
                    reload_is_major_action: true,
                }),
            },
            Self::Blunderbuss => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 2 },
                min_strength: None,
                two_handed: true,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 6,
                    long_range_yards: None,
                    reload_is_major_action: true,
                }),
            },
            Self::Musket => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(3), modifier: 1 },
                min_strength: Some(1),
                two_handed: true,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 24,
                    long_range_yards: Some(48),
                    reload_is_major_action: true,
                }),
            },
            Self::Pistol => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: None,
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 8,
                    long_range_yards: Some(16),
                    reload_is_major_action: true,
                }),
            },

            // BLUDGEONS
            Self::Mace => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 0 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },
            Self::Maul => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },
            Self::TwoHandedMaul => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 3 },
                min_strength: Some(3),
                two_handed: true,
                missile_properties: None,
            },

            // BOWS
            Self::Crossbow => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 1 },
                min_strength: Some(1),
                two_handed: false, // TODO: Is this two-handed?
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 30,
                    long_range_yards: Some(60),
                    reload_is_major_action: true,
                }),
            },
            Self::ShortBow => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: Some(-1),
                two_handed: true,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 16,
                    long_range_yards: Some(32),
                    reload_is_major_action: false,
                }),
            },
            Self::LongBow => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: Some(1),
                two_handed: true,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 26,
                    long_range_yards: Some(52),
                    reload_is_major_action: false,
                }),
            },

            // BRAWLING
            Self::Fist => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d3(1), modifier: 0 },
                min_strength: None,
                two_handed: false,
                missile_properties: None,
            },
            Self::Gauntlet => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d3(1), modifier: 1 },
                min_strength: None,
                two_handed: false,
                missile_properties: None,
            },
            Self::ImprovisedWeapon => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: -1 },
                min_strength: None,
                two_handed: false,  // I suppose this depends...
                missile_properties: None,
            },

            // DUELING
            Self::MainGauche => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: None,
                two_handed: false,
                missile_properties: None,
            },
            Self::Rapier => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: Some(0),
                two_handed: false,
                missile_properties: None,
            },
            Self::SpikedBuckler => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: -1 },
                min_strength: Some(-1),
                two_handed: false,
                missile_properties: None,
            },

            // HEAVY BLADES
            Self::BastardSword => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 1 },
                min_strength: Some(2),
                two_handed: false,  // TODO: This can be both!
                missile_properties: None,
            },
            Self::LongSword => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 0 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },
            Self::TwoHandedSword => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(3), modifier: 0 },
                min_strength: Some(3),
                two_handed: true,
                missile_properties: None,
            },

            // LANCES
            Self::HeavyLance => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(3), modifier: 1 },
                min_strength: Some(3),
                two_handed: false,
                missile_properties: None,
            },
            Self::JoustingLance => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 1 },
                min_strength: Some(0),
                two_handed: false,
                missile_properties: None,
            },
            Self::LightLance => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },

            // LIGHT BLADES
            Self::Dagger => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: None,
                two_handed: false,
                missile_properties: None,
            },
            Self::ShortSword => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 2 },
                min_strength: Some(-1),
                two_handed: false,
                missile_properties: None,
            },
            Self::ThrowingKnife => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 0 },
                min_strength: None,
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 6,
                    long_range_yards: Some(12),
                    reload_is_major_action: false,
                }),
            },

            // POLEARMS
            Self::Glaive => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 1 },
                min_strength: Some(1),
                two_handed: true,
                missile_properties: None,
            },
            Self::Halberd => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 4 },
                min_strength: Some(3),
                two_handed: true,
                missile_properties: None,
            },
            Self::MilitaryFork => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 1 },
                min_strength: Some(2),
                two_handed: true,
                missile_properties: None,
            },

            // SLINGS
            Self::Fustibale => WeaponProperties {  // TODO: Handle lead bullet variant (1d6 + 2)
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: Some(0),
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 14,
                    long_range_yards: Some(28),
                    reload_is_major_action: false,
                }),
            },
            Self::HuntingSling => WeaponProperties {  // TODO: Handle lead bullet variant (1d6 + 1)
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 0 },
                min_strength: Some(-1),
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 12,
                    long_range_yards: Some(24),
                    reload_is_major_action: false,
                }),
            },
            Self::Slingshot => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d3(1), modifier: 1 },
                min_strength: Some(-2),
                two_handed: false,  // TODO: ... *is* this two-handed?
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 10,
                    long_range_yards: Some(20),
                    reload_is_major_action: false,
                }),
            },

            // SPEARS
            Self::Spear => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 0 },
                min_strength: Some(0),
                two_handed: false,
                missile_properties: None,
            },
            Self::ThrowingSpear => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: Some(0),
                two_handed: false,
                missile_properties: Some(WeaponMissileProperties {
                    short_range_yards: 8,
                    long_range_yards: Some(16),
                    reload_is_major_action: false,
                }),
            },
            Self::TwoHandedSpear => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(2), modifier: 3 },
                min_strength: Some(1),
                two_handed: true,
                missile_properties: None,
            },

            // STAVES
            Self::Club => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 0 },
                min_strength: None,
                two_handed: false,
                missile_properties: None,
            },
            Self::Morningstar => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 3 },
                min_strength: Some(1),
                two_handed: false,
                missile_properties: None,
            },
            Self::Quarterstaff => WeaponProperties {
                damage: DiceWithMod { dice: Dice::d6(1), modifier: 1 },
                min_strength: None,
                two_handed: false,  // TODO: ... *is* this two-handed?
                missile_properties: None,
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            // AXES
            Self::BattleAxe => "Battle Axe",
            Self::ThrowingAxe => "Throwing Axe",
            Self::TwoHandedAxe => "Two-handed Axe",

            // BLACK POWDER
            Self::Arquebus => "Arquebus",
            Self::Blunderbuss => "Blunderbuss",
            Self::Musket => "Musket",
            Self::Pistol => "Pistol",

            // BLUDGEONS
            Self::Mace => "Mace",
            Self::Maul => "Maul",
            Self::TwoHandedMaul => "Two-handed Maul",

            // BOWS
            Self::Crossbow => "Crossbow",
            Self::ShortBow => "Short Bow",
            Self::LongBow => "Long Bow",

            // BRAWLING
            Self::Fist => "Fist",
            Self::Gauntlet => "Gauntlet",
            Self::ImprovisedWeapon => "Improvised Weapon",

            // DUELING
            Self::MainGauche => "Main Gauche",
            Self::Rapier => "Rapier",
            Self::SpikedBuckler => "Spiked Buckler",

            // HEAVY BLADES
            Self::BastardSword => "Bastard Sword",
            Self::LongSword => "Long Sword",
            Self::TwoHandedSword => "Two-handed Sword",

            // LANCES
            Self::HeavyLance => "Heavy Lance",
            Self::JoustingLance => "Jousting Lance",
            Self::LightLance => "Light Lance",

            // LIGHT BLADES
            Self::Dagger => "Dagger",
            Self::ShortSword => "Short Sword",
            Self::ThrowingKnife => "Throwing Knife",

            // POLEARMS
            Self::Glaive => "Glaive",
            Self::Halberd => "Halberd",
            Self::MilitaryFork => "Military Fork",

            // SLINGS
            Self::Fustibale => "Fustibale",
            Self::HuntingSling => "Hunting Sling",
            Self::Slingshot => "Slingshot",

            // SPEARS
            Self::Spear => "Spear",
            Self::ThrowingSpear => "Throwing Spear",
            Self::TwoHandedSpear => "Two-handed Spear",

            // STAVES
            Self::Club => "Club",
            Self::Morningstar => "Morningstar",
            Self::Quarterstaff => "Quarterstaff",
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponProperties {
    pub damage: DiceWithMod,
    pub min_strength: Option<i8>,
    pub two_handed: bool,
    pub missile_properties: Option<WeaponMissileProperties>,
    // TODO: cost
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMissileProperties {
    pub short_range_yards: u16,
    pub long_range_yards: Option<u16>,
    pub reload_is_major_action: bool, // TODO: make this some kind of enum
}

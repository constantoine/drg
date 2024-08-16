use super::Die;
use crate::damage::kind::Kind;

/// Fire dice.
pub const FIRE_DICE: Die = Die {
    single: 4,
    double: 1,
    special: 0,
    noop: 1,
    distributable: false,
    damage: Some(Kind::Fire),
};

/// Explosive dice.
pub const EXPLOSIVE_DICE: Die = Die {
    single: 3,
    double: 2,
    special: 1,
    noop: 0,
    distributable: false,
    damage: Some(Kind::Explosive),
};

/// Enemy dice not usable with weapons.
pub const ENEMY_DICE: Die = Die {
    single: 3,
    double: 0,
    special: 2,
    noop: 1,
    distributable: false,
    damage: Some(Kind::Enemy),
};

/// Melee dice.
pub const PICKAXE_DICE: Die = Die {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
    distributable: true,
    damage: Some(Kind::Melee),
};

/// Single is GOLD, special is NITRA.
pub const MINERAL_DICE: Die = Die {
    single: 2,
    double: 0,
    special: 2,
    noop: 2,
    distributable: false,
    damage: None,
};

/// Bullet dice. Very unreliable.
pub const BULLET_DICE: Die = Die {
    single: 4,
    double: 0,
    special: 0,
    noop: 2,
    distributable: false,
    damage: Some(Kind::Bullet),
};

/// Armor-piercing dice.
pub const PIERCING_DICE: Die = Die {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
    distributable: false,
    damage: Some(Kind::Piercing),
};

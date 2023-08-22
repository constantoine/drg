use super::Dice;
use crate::damage::kind::Kind;

/// Fire dice.
pub const FIRE_DICE: Dice = Dice {
    single: 4,
    double: 1,
    special: 0,
    noop: 1,
    distribuable: false,
    damage: Some(Kind::Fire),
};

/// Explosive dice.
pub const EXPLOSIVE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 1,
    noop: 0,
    distribuable: false,
    damage: Some(Kind::Explosive),
};

/// Ennemy dice not usable with weapons.
pub const ENNEMY_DICE: Dice = Dice {
    single: 3,
    double: 0,
    special: 2,
    noop: 1,
    distribuable: false,
    damage: Some(Kind::Ennemy),
};

/// Melee dice.
pub const PICKAXE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
    distribuable: true,
    damage: Some(Kind::Melee),
};

/// Single is GOLD, special is NITRA.
pub const MINERAL_DICE: Dice = Dice {
    single: 2,
    double: 0,
    special: 2,
    noop: 2,
    distribuable: false,
    damage: None,
};

/// Bullet dice. Very unreliable.
pub const BULLET_DICE: Dice = Dice {
    single: 4,
    double: 0,
    special: 0,
    noop: 2,
    distribuable: false,
    damage: Some(Kind::Bullet),
};

/// Armor-piercing dice.
pub const PIERCING_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
    distribuable: false,
    damage: Some(Kind::Piercing),
};

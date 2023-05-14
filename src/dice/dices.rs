use crate::dice::Dice;

pub const FIRE_DICE: Dice = Dice {
    single: 4,
    double: 1,
    special: 0,
    noop: 1,
};

pub const EXPLOSICE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 1,
    noop: 0,
};

pub const ENNEMY_DICE: Dice = Dice {
    single: 3,
    double: 0,
    special: 2,
    noop: 1,
};

pub const PICKAXE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
};

// Single is GOLD, special is NITRA;
pub const MINERAL_DICE: Dice = Dice {
    single: 2,
    double: 0,
    special: 2,
    noop: 2,
};

pub const BULLET_DICE: Dice = Dice {
    single: 4,
    double: 0,
    special: 0,
    noop: 2,
};

pub const PIERCING_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
};

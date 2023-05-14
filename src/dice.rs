#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiceResult {
    Single,
    Double,
    Special,
    Noop,
}

// Dice implementation is faces-number agnostic.
#[derive(Clone, Copy, Debug)]
struct Dice {
    single: u8,
    double: u8,
    special: u8,
    noop: u8,
}

const FIRE_DICE: Dice = Dice {
    single: 4,
    double: 1,
    special: 0,
    noop: 1,
};
const EXPLOSICE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 1,
    noop: 0,
};
const ENNEMY_DICE: Dice = Dice {
    single: 3,
    double: 0,
    special: 2,
    noop: 1,
};
const PICKAXE_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
};
// Single is GOLD, special is NITRA;
const MINERAL_DICE: Dice = Dice {
    single: 2,
    double: 0,
    special: 2,
    noop: 2,
};
const BULLET_DICE: Dice = Dice {
    single: 4,
    double: 0,
    special: 0,
    noop: 2,
};
const PIERCING_DICE: Dice = Dice {
    single: 3,
    double: 2,
    special: 0,
    noop: 1,
};

impl Dice {
    fn result(self, value: u8) -> DiceResult {
        match value {
            i if (1..=self.single).contains(&i) => DiceResult::Single,
            i if (self.single..=self.single + self.double).contains(&i) => DiceResult::Double,
            i if (self.single + self.double..=self.single + self.double + self.special)
                .contains(&i) =>
            {
                DiceResult::Special
            }
            i if (self.single + self.double + self.special
                ..=self.single + self.double + self.special + self.noop)
                .contains(&i) =>
            {
                DiceResult::Noop
            }
            _ => unreachable!("Dice result not in range."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire_dice() {
        let dice: Dice = FIRE_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn ennemy_dice() {
        let dice: Dice = ENNEMY_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn explosive_dice() {
        let dice: Dice = EXPLOSICE_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn pickaxe_dice() {
        let dice: Dice = PICKAXE_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn mineral_dice() {
        let dice: Dice = MINERAL_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn bullet_dice() {
        let dice: Dice = BULLET_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }

    #[test]
    fn piercing_dice() {
        let dice: Dice = PIERCING_DICE;

        let results: Vec<DiceResult> = (1..=6).map(|i| dice.result(i)).collect();

        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Noop)
                .count() as u8,
            dice.noop
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Single)
                .count() as u8,
            dice.single
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Double)
                .count() as u8,
            dice.double
        );
        assert_eq!(
            results
                .iter()
                .filter(|&res| *res == DiceResult::Special)
                .count() as u8,
            dice.special
        );
    }
}

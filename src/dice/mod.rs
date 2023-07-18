pub mod dices;

use crate::damage::kind::Kind;
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiceResult {
    Single,
    Double,
    Special,
    Noop,
}

// Dice implementation is faces-number agnostic.
// distribuable property denotes if one dice's result can be split.
// eg, Melee dice
#[derive(Clone, Copy, Debug)]
pub struct Dice {
    single: u8,
    double: u8,
    special: u8,
    noop: u8,
    distribuable: bool,
    damage: Option<Kind>,
}

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

    /// Throw a dice.
    pub fn throw(self, rng: &mut SmallRng) -> DiceResult {
        self.result(rng.gen_range(1..=6))
    }
}

#[cfg(test)]
mod tests {
    use super::dices::*;
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
        let dice: Dice = EXPLOSIVE_DICE;

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

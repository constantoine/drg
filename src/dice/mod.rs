/// [Die] constants.
pub mod dices;

use crate::damage::kind::Kind;
use rand::rngs::SmallRng;
use rand::Rng;

/// Throw result containing which dice face was up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiceResult {
    /// One hit.
    Single,
    /// Two hits.
    Double,
    /// Special hit.
    Special,
    /// Missed.
    Noop,
}

/// Die implementation is faces-number agnostic.
/// distributable property denotes if one die's result can be split.
/// eg, Melee dice.
#[derive(Clone, Copy, Debug)]
pub struct Die {
    /// Number of faces whose result is 1 hit.
    single: u8,
    /// Number of faces whose result is 2 hits.
    double: u8,
    /// Number of faces whose result is special damage.
    special: u8,
    /// Number of faces whose result is 0 hits (missed).
    noop: u8,
    /// If damaged can be split between enemies.
    distributable: bool,
    /// The kind of damage dealt, if relevant.
    damage: Option<Kind>,
}

impl Die {
    /// Super complex and expensive calculation to map a number to a face.
    /// On a given dice, the same input produces the same output.
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

    /// Throw a die using rng as a source of randomness.
    pub fn throw(self, rng: &mut SmallRng) -> DiceResult {
        self.result(rng.gen_range(1..=(self.single + self.double + self.noop + self.special)))
    }
}

#[cfg(test)]
mod tests {
    use super::dices::*;
    use super::*;

    #[test]
    fn fire_dice() {
        let dice: Die = FIRE_DICE;

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
    fn enemy_dice() {
        let dice: Die = ENEMY_DICE;

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
        let dice: Die = EXPLOSIVE_DICE;

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
        let dice: Die = PICKAXE_DICE;

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
        let dice: Die = MINERAL_DICE;

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
        let dice: Die = BULLET_DICE;

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
        let dice: Die = PIERCING_DICE;

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

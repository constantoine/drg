/// Different kinds of damage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Fire,
    Explosive,
    Enemy,
    Piercing,
    Bullet,
    Melee,
}

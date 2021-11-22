use rand::prelude::{thread_rng, Rng};

use super::{Game, GameBuilder};

#[derive(Debug, PartialEq)]
pub enum Variant {
    Classic,
    Advanced,
}

impl Default for Variant {
    fn default() -> Self { Self::Classic }
}

impl From<Variant> for GameBuilder {
    fn from(variant: Variant) -> Self {
        use Variant::*;

        match variant {
            Classic => GameBuilder::default(),
            _ => unimplemented!(),
        }
    }
}

impl From<Variant> for Game {
    fn from(variant: Variant) -> Self { GameBuilder::from(variant).into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_classic() {
        assert_eq!(Variant::default(), Variant::Classic);
    }
}

use rand::prelude::{thread_rng, Rng};

use super::{Game, Variant};

#[derive(Default)]
pub struct GameBuilder {
    pub pegs:              Option<Vec<u8>>,
    pub peg_range:         Option<u8>,
    pub peg_count:         Option<u8>,
    pub max_guesses:       Option<u8>,
    pub unlimited_guesses: bool,
}

impl GameBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn build(self) -> Game { self.into() }

    pub fn pegs(mut self, pegs: &[u8]) -> Self {
        self.pegs = pegs.to_owned().into();
        self
    }

    pub fn peg_range(mut self, range: u8) -> Self {
        self.peg_range = Some(range);
        self
    }

    pub fn peg_count(mut self, count: u8) -> Self {
        self.peg_count = Some(count);
        self
    }

    pub fn max_guesses(mut self, guesses: u8) -> Self {
        self.max_guesses = Some(guesses);
        self
    }

    pub fn unlimited_guesses(mut self, unlimited_guesses: bool) -> Self {
        self.unlimited_guesses = unlimited_guesses;
        self
    }

    fn calculate_pegs(&self) -> Vec<u8> {
        let peg_count = self.peg_count.unwrap_or(4);

        if let Some(pegs) = &self.pegs {
            if pegs.len() != peg_count as usize {
                panic!(
                    "Trying to build a Game with pegs {:?} and peg_count {}",
                    pegs, peg_count
                );
            }
            pegs.clone()
        }
        else {
            let mut rng = thread_rng();

            (0..peg_count)
                .map(|_| {
                    let rand: u8 = rng.gen();
                    rand % self.peg_range.unwrap_or(6)
                })
                .collect::<Vec<_>>()
        }
    }
}

impl From<GameBuilder> for Game {
    fn from(builder: GameBuilder) -> Self {
        let max_guesses = {
            if !builder.unlimited_guesses {
                builder.max_guesses.or(Some(12))
            }
            else {
                None
            }
        };

        Self {
            pegs: builder.calculate_pegs(),
            guesses: Vec::new(),
            max_guesses,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peg_count_is_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().peg_count(i as u8).build();
            assert_eq!(game.pegs().len(), i);
        })
    }

    #[test]
    fn peg_range_is_respected() {
        // This has a small chance of failing even with correct implementations,
        // however it's so unlikely that it's negligible.

        (1..8).for_each(|i| {
            let game = GameBuilder::new().peg_count(255).peg_range(i).build();

            // Assert that all pegs are within the given range.
            game.pegs().iter().for_each(|peg| {
                assert!((0..i).contains(peg));
            });

            // Assert that every peg appears at least once.
            (0..i).for_each(|i| {
                assert!(game.pegs().iter().any(|peg| { *peg == i }))
            });
        })
    }

    #[test]
    fn pegs_are_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().pegs(&[i, i, i, i]).build();
            assert_eq!(game.pegs(), [i, i, i, i]);
        })
    }

    #[test]
    fn max_guesses_is_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().max_guesses(i).build();
            assert_eq!(game.max_guesses.unwrap(), i);
        });

        // default should be 12
        let game = GameBuilder::new().build();
        assert_eq!(game.max_guesses.unwrap(), 12);
    }

    #[test]
    fn unlimited_guesses_is_respected() {
        let game = GameBuilder::new().unlimited_guesses(true).build();
        assert!(game.max_guesses.is_none());
    }

    #[test]
    #[should_panic]
    fn panics_for_wrong_number_of_pins() {
        GameBuilder::new().pegs(&[1, 2, 3]).peg_count(2).build();
    }
}

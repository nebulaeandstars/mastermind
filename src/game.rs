use rand::prelude::{thread_rng, Rng};


pub struct Game {
    pegs:        Vec<u8>,
    guesses:     Vec<Vec<u8>>,
    max_guesses: Option<u8>,
}

impl Game {
    pub fn pegs(&self) -> &[u8] { &self.pegs }
    pub fn guesses(&self) -> &[Vec<u8>] { &self.guesses }

    pub fn guess(&mut self, guess: &[u8]) -> Result<(u8, u8), GuessError> {
        if let Some(max_guesses) = self.max_guesses {
            if self.guesses.len() == max_guesses as usize {
                return Err(GuessError::NoGuessesLeft);
            }
        }

        self.guesses.push(guess.to_owned());
        Ok(self.hits(self.guesses.len() - 1).unwrap())
    }

    pub fn hits(&self, index: usize) -> Option<(u8, u8)> {
        self.guesses.get(index).map(|guess_pegs| {
            let mut hits = 0;
            let mut near_hits = 0;

            let guess = guess_pegs.clone();
            let real = self.pegs.clone();

            let mut guess = guess.iter().map(|p| Some(p)).collect::<Vec<_>>();
            let mut real = real.iter().map(|p| Some(p)).collect::<Vec<_>>();

            for (i, real_peg) in real.iter_mut().enumerate() {
                if guess[i] == *real_peg {
                    guess[i] = None;
                    *real_peg = None;
                    hits += 1;
                }
            }

            for (i, real_peg) in real.iter_mut().enumerate() {
                for (j, guess_peg) in guess.iter_mut().enumerate() {
                    if real_peg.is_some() && *real_peg == *guess_peg && i != j {
                        *guess_peg = None;
                        *real_peg = None;
                        near_hits += 1;
                    }
                }
            }

            (hits, near_hits)
        })
    }
}

impl Default for Game {
    fn default() -> Self { GameBuilder::default().into() }
}

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

#[derive(Debug, PartialEq)]
pub enum GuessError {
    NoGuessesLeft,
}

impl From<Variant> for Game {
    fn from(variant: Variant) -> Self { GameBuilder::from(variant).into() }
}

#[cfg(test)]
mod tests {
    use rand::prelude::{thread_rng, Rng};

    use super::*;

    #[test]
    fn variant_default_is_classic() {
        assert_eq!(Variant::default(), Variant::Classic);
    }

    #[test]
    fn builder_peg_count_is_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().peg_count(i as u8).build();
            assert_eq!(game.pegs().len(), i);
        })
    }

    #[test]
    fn builder_peg_range_is_respected() {
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
    fn builder_pegs_are_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().pegs(&[i, i, i, i]).build();
            assert_eq!(game.pegs(), [i, i, i, i]);
        })
    }

    #[test]
    fn builder_max_guesses_is_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().max_guesses(i).build();
            assert_eq!(game.max_guesses.unwrap(), i);
        });

        // default should be 12
        let game = GameBuilder::new().build();
        assert_eq!(game.max_guesses.unwrap(), 12);
    }

    #[test]
    fn builder_unlimited_guesses_is_respected() {
        let game = GameBuilder::new().unlimited_guesses(true).build();
        assert!(game.max_guesses.is_none());
    }

    #[test]
    #[should_panic]
    fn builder_panics_for_wrong_number_of_pins() {
        GameBuilder::new().pegs(&[1, 2, 3]).peg_count(2).build();
    }

    #[test]
    fn game_pegs_returns_correct_pegs() {
        let mut rng = thread_rng();
        let mut f = || rng.gen::<u8>() & 6;

        for _ in 0..50 {
            let (a, b, c, d) = (f(), f(), f(), f());
            let game = GameBuilder::new().pegs(&[a, b, c, d]).build();
            assert_eq!(game.pegs(), &[a, b, c, d]);
        }
    }

    #[test]
    fn game_limits_guesses() {
        let mut game = GameBuilder::new().max_guesses(2).build();
        assert!(game.guess(&[1, 2, 3, 4]).is_ok());
        assert!(game.guess(&[1, 2, 3, 4]).is_ok());
        assert!(game.guess(&[1, 2, 3, 4]).is_err());
    }

    #[test]
    fn game_returns_accurate_hits() {
        let mut game = GameBuilder::new().pegs(&[1, 1, 2, 2]).build();
        assert_eq!(game.guess(&[1, 1, 1, 1]), Ok((2, 0)));
        assert_eq!(game.guess(&[0, 2, 1, 4]), Ok((0, 2)));
        assert_eq!(game.guess(&[1, 2, 3, 1]), Ok((1, 2)));
        assert_eq!(game.guess(&[1, 1, 2, 2]), Ok((4, 0)));

        let mut game = GameBuilder::new().pegs(&[1, 5, 6, 3]).build();
        assert_eq!(game.guess(&[0, 0, 0, 0]), Ok((0, 0)));
        assert_eq!(game.guess(&[0, 0, 1, 0]), Ok((0, 1)));
        assert_eq!(game.guess(&[0, 5, 1, 0]), Ok((1, 1)));
        assert_eq!(game.guess(&[3, 5, 1, 0]), Ok((1, 2)));
    }
}

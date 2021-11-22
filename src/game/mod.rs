mod builder;
mod variant;

pub use builder::GameBuilder;
pub use variant::Variant;

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

#[derive(Debug, PartialEq)]
pub enum GuessError {
    NoGuessesLeft,
}

#[cfg(test)]
mod tests {
    use rand::prelude::{thread_rng, Rng};

    use super::*;

    #[test]
    fn pegs_returns_correct_pegs() {
        let mut rng = thread_rng();
        let mut f = || rng.gen::<u8>() & 6;

        for _ in 0..50 {
            let (a, b, c, d) = (f(), f(), f(), f());
            let game = GameBuilder::new().pegs(&[a, b, c, d]).build();
            assert_eq!(game.pegs(), &[a, b, c, d]);
        }
    }

    #[test]
    fn guesses_are_limited() {
        let mut game = GameBuilder::new().max_guesses(2).build();
        assert!(game.guess(&[1, 2, 3, 4]).is_ok());
        assert!(game.guess(&[1, 2, 3, 4]).is_ok());
        assert!(game.guess(&[1, 2, 3, 4]).is_err());
    }

    #[test]
    fn hits_returns_accurate_hits() {
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

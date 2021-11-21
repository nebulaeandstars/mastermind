use rand::prelude::*;

#[derive(Default)]
pub struct GameBuilder {
    pub pegs:           Option<Vec<u8>>,
    pub peg_range:      Option<u8>,
    pub peg_count:      Option<u8>,
    pub max_rows:       Option<u8>,
    pub unlimited_rows: bool,
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

    pub fn max_rows(mut self, rows: u8) -> Self {
        self.max_rows = Some(rows);
        self
    }

    pub fn unlimited_rows(mut self, unlimited_rows: bool) -> Self {
        self.unlimited_rows = unlimited_rows;
        self
    }

    fn calculate_pegs(&self) -> Vec<u8> {
        if let Some(pegs) = &self.pegs {
            pegs.clone()
        }
        else {
            let mut rng = thread_rng();

            (0..self.peg_count.unwrap_or(4))
                .map(|_| {
                    let rand: u8 = rng.gen();
                    rand % self.peg_range.unwrap_or(6)
                })
                .collect::<Vec<_>>()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pegs:     Vec<u8>,
    rows:     Vec<u8>,
    max_rows: Option<u8>,
}

impl Game {
    pub fn pegs(&self) -> &[u8] { &self.pegs }
}

impl From<GameBuilder> for Game {
    fn from(builder: GameBuilder) -> Self {
        let max_rows = {
            if !builder.unlimited_rows {
                builder.max_rows.or(Some(12))
            }
            else {
                None
            }
        };

        Self { pegs: builder.calculate_pegs(), rows: Vec::new(), max_rows }
    }
}

impl Default for Game {
    fn default() -> Self { GameBuilder::default().into() }
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    Classic,
    Advanced,
}

impl Default for Variant {
    fn default() -> Self { Self::Classic }
}

impl From<Variant> for Game {
    fn from(variant: Variant) -> Self {
        use Variant::*;

        match variant {
            Classic => Game::default(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_variant_is_classic() {
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
    fn bulder_max_rows_is_respected() {
        (1..8).for_each(|i| {
            let game = GameBuilder::new().max_rows(i).build();
            assert_eq!(game.max_rows.unwrap(), i);
        });

        // default should be 12
        let game = GameBuilder::new().build();
        assert_eq!(game.max_rows.unwrap(), 12);
    }

    #[test]
    fn bulder_unlimited_rows_is_respected() {
        let game = GameBuilder::new().unlimited_rows(true).build();
        assert!(game.max_rows.is_none());
    }
}

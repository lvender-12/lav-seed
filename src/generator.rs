use crate::{
    LavError,
    config::Config,
    transform::{DefaultTransform, Transform},
};

/// A deterministic numeric ID generator based on affine permutation.
///
/// This generator produces unique IDs within a fixed range without collisions.
/// The output is deterministic based on the configuration and seed.
///
/// # Properties
/// - Collision-free within configured range
/// - Fast O(1) generation
/// - Fully deterministic output
/// - Supports custom transform strategies
///
/// # Example
/// ```rust
///     use lav_seed::Generator;
///     let mut generator = Generator::build(0)
///         .min_seed(1)
///         .max_seed(1_000_000)
///         .build()
///         .unwrap();
///     let id = generator.generate().unwrap();
///     println!("{}", id);
/// ```
pub struct Generator {
    config: Config,
    counter: u64,
    transform: Box<dyn Transform>,
}

impl Generator {
    pub fn from_config(config: Config) -> Self {
        let initial_counter = config.initial_counter;

        Self {
            config,
            counter: initial_counter,
            transform: Box::new(crate::DefaultTransform),
        }
    }

    pub fn build(initial_counter: u64) -> GeneratorBuilder {
        GeneratorBuilder::new(initial_counter)
    }

    #[inline]
    fn range(&self) -> u64 {
        self.config.max - self.config.min + 1
    }

    /// # Example
    /// ```rust
    ///     use lav_seed::Generator;
    ///     let mut generator = Generator::build(0)
    ///         .min_seed(1)
    ///         .max_seed(1_000_000)
    ///         .build()
    ///         .unwrap();
    ///     let id = generator.generate().unwrap();
    ///     println!("{}", id);
    /// ```
    pub fn generate(&mut self) -> Result<u64, LavError> {
        let range = self.range();

        if self.counter >= range {
            return Err(LavError::Exhausted);
        }

        let result = self.transform.apply(self.counter, range, self.config.key);

        let id = result + self.config.min;

        self.counter = self
            .counter
            .checked_add(1)
            .ok_or(LavError::CounterOverflow(self.counter))?;

        Ok(id)
    }

    /// # Example
    /// ```rust
    ///     use lav_seed::Generator;
    ///     let mut generator = Generator::build(0)
    ///         .min_seed(1)
    ///         .max_seed(1_000_000)
    ///         .build()
    ///         .unwrap();
    ///     let peeked = generator.peek().unwrap();
    ///     let actual = generator.generate().unwrap();
    ///     assert_eq!(peeked, actual);
    /// ```
    pub fn peek(&self) -> Result<u64, LavError> {
        let range = self.range();

        if self.counter >= range {
            return Err(LavError::Exhausted);
        }

        let result = self.transform.apply(self.counter, range, self.config.key);

        Ok(result + self.config.min)
    }

    /// Resets the counter back to zero.
    pub fn reset(&mut self) {
        self.counter = 0;
    }

    /// Jumps the counter forward by `amount`.
    pub fn jump(&mut self, amount: u64) -> Result<(), LavError> {
        self.counter = self
            .counter
            .checked_add(amount)
            .ok_or(LavError::CounterOverflow(self.counter))?;

        Ok(())
    }

    /// Returns the current counter value.
    pub fn counter(&self) -> u64 {
        self.counter
    }

    /// Returns how many IDs are remaining before exhaustion.
    pub fn remaining(&self) -> u64 {
        self.range().saturating_sub(self.counter)
    }

    /// Returns `true` if all IDs in the range have been generated.
    pub fn exhausted(&self) -> bool {
        self.counter >= self.range()
    }

    /// Validates whether `id` is within the configured range.
    pub fn validate(&self, id: u64) -> Result<(), LavError> {
        if id < self.config.min || id > self.config.max {
            return Err(LavError::InvalidSeed(id));
        }

        Ok(())
    }

    /// Returns `true` if `id` is within the configured range.
    pub fn contains(&self, id: u64) -> bool {
        id >= self.config.min && id <= self.config.max
    }

    /// Returns a reference to the current configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub struct GeneratorBuilder {
    initial_counter: u64,
    config: Config,
    transform: Box<dyn Transform>,
}

impl GeneratorBuilder {
    pub fn new(initial_counter: u64) -> Self {
        Self {
            initial_counter,
            config: Config::default(),
            transform: Box::new(DefaultTransform),
        }
    }

    pub fn min_seed(mut self, min: u64) -> Self {
        self.config.min = min;
        self
    }

    pub fn max_seed(mut self, max: u64) -> Self {
        self.config.max = max;
        self
    }

    pub fn key(mut self, key: u64) -> Self {
        self.config.key = key;
        self
    }

    pub fn transform(mut self, t: impl Transform + 'static) -> Self {
        self.transform = Box::new(t);
        self
    }

    pub fn build(self) -> Result<Generator, LavError> {
        if self.config.min >= self.config.max {
            return Err(LavError::InvalidRange {
                min: self.config.min,
                max: self.config.max,
            });
        }

        Ok(Generator {
            config: self.config,
            counter: self.initial_counter,
            transform: self.transform,
        })
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.generate().ok()
    }
}

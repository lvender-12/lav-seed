use crate::LavError;

/// Configuration for `Lav-Seed` generator.
///
/// Defines range, seed key, and starting counter.
pub struct Config {
    pub(crate) min: u64,
    pub(crate) max: u64,
    pub(crate) key: u64,
    pub(crate) initial_counter: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min: 100_000_000,
            max: 999_999_999,
            key: 0x3C6EF35F,
            initial_counter: 0,
        }
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
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

    pub fn initial_counter(mut self, counter: u64) -> Self {
        self.config.initial_counter = counter;
        self
    }

    pub fn build(self) -> Result<Config, LavError> {
        if self.config.min >= self.config.max {
            return Err(LavError::InvalidRange {
                min: self.config.min,
                max: self.config.max,
            });
        }
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

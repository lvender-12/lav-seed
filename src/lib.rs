mod config;
mod error;
mod generator;
mod transform;

pub use error::LavError;
pub use generator::Generator;
pub use transform::{ClosureTransform, DefaultTransform, Transform};

use crate::config::ConfigBuilder;

#[allow(dead_code)]
pub fn example() {
    let config = ConfigBuilder::new()
        .min_seed(100)
        .max_seed(1_000_000)
        .key(42)
        .initial_counter(0)
        .build()
        .unwrap();

    let mut g = Generator::from_config(config);

    let _id = g.generate().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate() {
        let mut g = Generator::new(0)
            .min_seed(100_000_000)
            .max_seed(999_999_999)
            .build()
            .unwrap();

        for _ in 0..10 {
            let id = g.generate().unwrap();
            println!("{}", id);
        }
    }

    #[test]
    fn test_unique_bulk() {
        let mut g = Generator::new(0)
            .min_seed(100_000_000)
            .max_seed(999_999_999)
            .key(12345)
            .build()
            .unwrap();

        let mut seen = HashSet::new();

        for i in 0..1_000_000 {
            let id = g.generate().unwrap();

            assert!(seen.insert(id), "Collision at counter {} => {}", i, id);
        }
    }

    #[test]
    fn test_exhausted() {
        let mut g = Generator::new(0).min_seed(1).max_seed(3).build().unwrap();

        assert!(g.generate().is_ok());
        assert!(g.generate().is_ok());
        assert!(g.generate().is_ok());

        assert!(matches!(g.generate(), Err(LavError::Exhausted)));
    }

    #[test]
    fn test_peek() {
        let mut g = Generator::new(0)
            .min_seed(1000)
            .max_seed(9999)
            .build()
            .unwrap();

        let peeked = g.peek().unwrap();
        let actual = g.generate().unwrap();

        assert_eq!(peeked, actual);
    }

    #[test]
    fn test_counter() {
        let mut g = Generator::new(0).build().unwrap();

        assert_eq!(g.counter(), 0);

        g.generate().unwrap();
        g.generate().unwrap();

        assert_eq!(g.counter(), 2);
    }

    #[test]
    fn test_remaining() {
        let mut g = Generator::new(0).min_seed(1).max_seed(10).build().unwrap();

        assert_eq!(g.remaining(), 10);

        g.generate().unwrap();

        assert_eq!(g.remaining(), 9);
    }

    #[test]
    fn test_reset() {
        let mut g = Generator::new(0).build().unwrap();

        g.generate().unwrap();
        g.generate().unwrap();

        assert_eq!(g.counter(), 2);

        g.reset();

        assert_eq!(g.counter(), 0);
    }

    #[test]
    fn test_contains() {
        let g = Generator::new(0)
            .min_seed(100)
            .max_seed(200)
            .build()
            .unwrap();

        assert!(g.contains(150));
        assert!(g.contains(100));
        assert!(g.contains(200));

        assert!(!g.contains(99));
        assert!(!g.contains(201));
    }

    #[test]
    fn test_validate() {
        let g = Generator::new(0)
            .min_seed(100)
            .max_seed(200)
            .build()
            .unwrap();

        assert!(g.validate(150).is_ok());

        assert!(matches!(g.validate(999), Err(LavError::InvalidSeed(_))));
    }

    #[test]
    fn test_iterator() {
        let g = Generator::new(0).min_seed(1).max_seed(5).build().unwrap();

        let ids: Vec<u64> = g.take(5).collect();

        assert_eq!(ids.len(), 5);

        let unique: HashSet<u64> = ids.iter().copied().collect();

        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn test_jump() {
        let mut g = Generator::new(0).min_seed(1).max_seed(100).build().unwrap();

        g.jump(10).unwrap();

        assert_eq!(g.counter(), 10);
    }

    #[test]
    fn test_exhausted_state() {
        let mut g = Generator::new(0).min_seed(1).max_seed(2).build().unwrap();

        assert!(!g.exhausted());

        g.generate().unwrap();
        g.generate().unwrap();

        assert!(g.exhausted());
    }

    #[test]
    fn test_full_range_no_collision() {
        let min = 1u64;
        let max = 1_000_000u64;

        let mut g = Generator::new(0)
            .min_seed(min)
            .max_seed(max)
            .key(12345)
            .build()
            .unwrap();

        let range = (max - min + 1) as usize;

        let mut seen = vec![false; range];

        for i in 0..range {
            let id = g.generate().unwrap();

            assert!(id >= min && id <= max, "Out of range at {} => {}", i, id);

            let idx = (id - min) as usize;

            assert!(!seen[idx], "Collision at counter {} => {}", i, id);

            seen[idx] = true;
        }

        assert!(seen.iter().all(|v| *v));

        assert!(matches!(g.generate(), Err(LavError::Exhausted)));
    }
}

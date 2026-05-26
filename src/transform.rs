/// Core trait for ID transformation strategy.
///
/// Implement this to customize how IDs are generated from the counter.
///
/// # Example
/// ```rust
/// use lav_seed::transform::{Transform, ClosureTransform};
///
/// let t = ClosureTransform::new(|counter, range, key| {
///     (counter + key) % range
/// });
/// ```
pub trait Transform: Send + Sync {
    /// Applies the transformation to produce an ID offset within `range`.
    fn apply(&self, counter: u64, range: u64, key: u64) -> u64;
}

/// Default transformation using affine permutation.
///
/// Uses `f(x) = (A * x + key) % range` where `A = 1_000_000_007`.
pub struct DefaultTransform;

impl Transform for DefaultTransform {
    fn apply(&self, counter: u64, range: u64, key: u64) -> u64 {
        const A: u64 = 1_000_000_007;

        counter.wrapping_mul(A).wrapping_add(key) % range
    }
}

/// A transform backed by a closure, for custom ID generation logic.
///
/// # Example
/// ```rust
/// use lav_seed::transform::ClosureTransform;
///
/// let t = ClosureTransform::new(|counter, range, key| {
///     (counter + key) % range
/// });
/// ```
pub struct ClosureTransform<F>
where
    F: Fn(u64, u64, u64) -> u64 + Send + Sync,
{
    func: F,
}

impl<F> ClosureTransform<F>
where
    F: Fn(u64, u64, u64) -> u64 + Send + Sync,
{
    /// Creates a new `ClosureTransform` from the given closure.
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Transform for ClosureTransform<F>
where
    F: Fn(u64, u64, u64) -> u64 + Send + Sync,
{
    fn apply(&self, counter: u64, range: u64, key: u64) -> u64 {
        (self.func)(counter, range, key)
    }
}

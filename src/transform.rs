pub trait Transform: Send + Sync {
    fn apply(&self, counter: u64, range: u64, key: u64) -> u64;
}

pub struct DefaultTransform;

impl Transform for DefaultTransform {
    fn apply(&self, counter: u64, range: u64, key: u64) -> u64 {
        const A: u64 = 1_000_000_007;

        counter.wrapping_mul(A).wrapping_add(key) % range
    }
}

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

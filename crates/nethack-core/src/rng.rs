use rand::SeedableRng;
use rand::rngs::StdRng;

/// NetHack random number generator wrapper
pub struct Rng {
    inner: StdRng,
}

impl Rng {
    /// Create a new seeded RNG
    pub fn new(seed: u64) -> Self {
        Self {
            inner: StdRng::seed_from_u64(seed),
        }
    }

    /// Get a random integer in range [0, max)
    pub fn next_u32(&mut self, max: u32) -> u32 {
        use rand::Rng as _;
        self.inner.gen_range(0..max)
    }
}

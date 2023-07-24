use nanorand::{tls_rng, Rng};

pub trait Seeded: Sized {
    fn seed_from_u64(seed: u64) -> Self;

    fn random() -> Self {
        let mut rng = tls_rng();
        Self::seed_from_u64(rng.generate())
    }
}

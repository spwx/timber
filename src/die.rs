use rand::distributions::{Distribution, Uniform};

pub struct Die(pub i32);

impl Die {
    pub fn roll(&self) -> i32 {
        let between = Uniform::from(1..=self.0);

        let mut rng = rand::thread_rng();
        between.sample(&mut rng)
    }
}

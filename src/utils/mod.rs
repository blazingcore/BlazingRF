pub mod server;

use rand::{Rng, distr::Alphanumeric};

pub fn generate_secret() -> String {
    let length = 32;
    let secret: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    secret
}

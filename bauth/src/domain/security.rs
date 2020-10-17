use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_random_string(len: usize) -> String {
    return thread_rng().sample_iter(Alphanumeric).take(len).collect();
}

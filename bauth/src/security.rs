use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_random_string(len: usize) -> String {
    thread_rng().sample_iter(Alphanumeric).take(len).collect()
}

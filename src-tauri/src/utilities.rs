use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen_rand_id() -> u64 {
    rand::random()
}

pub fn get_curr_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}
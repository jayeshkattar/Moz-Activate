use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}

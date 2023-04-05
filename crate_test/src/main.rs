
use std::time::{
    Duration, SystemTime, UNIX_EPOCH
};

#[derive(Debug)]
struct Test {
    #[allow(dead_code)]
    t: SystemTime
}


fn main() {
    let now = SystemTime::now() + Duration::from_secs(15 * 60);
    let u = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("{:?}", u);
    let serial = Test{t: now};
    println!("{:?}", serial);
    // sleep(Duration::from_secs(1));
    // println!("{:?}", now.checked_duration_since(Instant::now()));
}

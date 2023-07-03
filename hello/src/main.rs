use time::macros::{date, time};
use time::{Duration, PrimitiveDateTime as DateTime};

fn main() {
    let a = DateTime::new(date!(2020 - 01 - 01), time!(0:00));
    let b = a + Duration::milliseconds(20);
    println!("{:?}", b);
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}

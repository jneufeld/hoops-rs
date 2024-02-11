use time::{Duration, PrimitiveDateTime as DateTime};

/// Returns a `DateTime` one billion seconds after `start`.
pub fn after(start: DateTime) -> DateTime {
    let one_giga_second = Duration::seconds(1_000_000_000);
    start.checked_add(one_giga_second).unwrap()
}

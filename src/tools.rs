use time::{Weekday};

pub fn weekday_iso(weekday: Weekday) -> u8 {
    match weekday { 
        time::Weekday::Monday => 1,
        time::Weekday::Tuesday => 2,
        time::Weekday::Wednesday => 3,
        time::Weekday::Thursday => 4,
        time::Weekday::Friday => 5,
        time::Weekday::Saturday => 6,
        time::Weekday::Sunday => 7,
    }
}
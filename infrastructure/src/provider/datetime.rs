use application::abstractions::DateTimeProvider;
use chrono::DateTime as _DateTime;

pub struct DateTime;

impl DateTimeProvider for DateTime {
    type DateTime = _DateTime<chrono::offset::Utc>;

    fn utc_now(&self) -> Self::DateTime {
        chrono::Utc::now()
    }
    fn add_minutes(&self, duration: i64) -> Self::DateTime {
        self.utc_now() + chrono::Duration::minutes(duration)
    }
}

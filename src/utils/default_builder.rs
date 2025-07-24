use chrono::DateTime;

pub trait DefaultRequestBuilder {
    fn max_results(self, max: i64) -> Self;
    fn page_token(self, token: &str) -> Self;
    fn time_min(self, time_min: DateTime<chrono::Utc>) -> Self;
    fn time_max(self, time_max: DateTime<chrono::Utc>) -> Self;
}

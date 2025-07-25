use chrono::NaiveDate;

pub fn parse_date(s: &str) -> std::result::Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format '{}': {}", s, e))
}


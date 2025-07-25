use std::result::Result;

use anyhow::Ok;
use chrono::NaiveDate;

// cargo add chrono --features serde
// NaiveDate::parse_from_str は Result<NaiveDate, ParseError> -> Result<NaiveDate, String>
fn parse_date(s: &str) -> Result<NaiveDate, String> {
    // print!("{:?}", NaiveDate::from_ymd_opt(2024, 7, 25).unwrap());

    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format '{}': {}", s, e))
}

pub fn date_main() {
    let result = parse_date("2024-07-25");
    if result.is_ok() {
        println!("{:?}", result)
    }
}

// fn divide(x: i32, y: i32) -> Result<i32, String> {
//     if y == 0 {
//         Err("division by zero".to_string())
//             .map_err(|e| format!("Math error: {}", e))
//     } else {
//         Ok(x / y)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_parse_valid_date() {
        let result = parse_date("2024-07-25");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NaiveDate::from_ymd_opt(2024, 7, 25).unwrap());
    }

    #[test]
    fn test_parse_valid_format() {
        let result = parse_date("07/25/2024");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date format"))
    }

    #[test]
    fn test_parse_nonexistent_date() {
        let result = parse_date("2024-02-30");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date format"))
    }
}

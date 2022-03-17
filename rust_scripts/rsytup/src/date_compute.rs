use chrono::{Weekday, NaiveDate, Datelike};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coming_weekdays() {
        let thu = parse_iso_date("2021-09-02").unwrap();
        let fri = coming_weekday(thu, Weekday::Fri);
        assert_eq!(fri, parse_iso_date("2021-09-03").unwrap());
    }

    fn test_add_weeks() {
        let thu = parse_iso_date("2021-09-02").unwrap();
        let fri = add_weeks(thu, 1);
        assert_eq!(fri, parse_iso_date("2021-09-09").unwrap());
        let already_fri = parse_iso_date("2021-09-03").unwrap();
        let fri = add_weeks(already_fri, 1);
        assert_eq!(fri, parse_iso_date("2021-09-09").unwrap());  
    }
}

/// NaiveDate from ISO 8601 to String
pub fn parse_iso_date(string_date: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(string_date, "%Y-%m-%d")
}

/// Returns the next upcoming day. If we pass it thuesday it will return when the next thursday is
pub fn coming_weekday(start: NaiveDate, weekday: Weekday) -> NaiveDate {
     if start.weekday() == weekday {
         return start + chrono::Duration::weeks(1);
     }

     for day in start.iter_days().take(6) {
         if day.weekday() == weekday {
             return day;
         }
     }

     start
}

/// Weeks after start_date
pub fn add_weeks(start: NaiveDate, weeks: u8) -> NaiveDate {
    start + chrono::Duration::weeks(weeks as i64)
}
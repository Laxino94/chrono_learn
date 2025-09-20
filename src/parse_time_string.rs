#![allow(dead_code)]
pub mod parse_time_string {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
    use regex::Regex;

    pub fn to_datetime(input: &str) -> Result<DateTime<Utc>, String> {
        // YYYY-MM-DD
        let re_date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        if re_date.is_match(input) {
            let date = NaiveDate::parse_from_str(input, "%Y-%m-%d")
                .map_err(|e| e.to_string())?;
            return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        // YYYY-MM-DD HH:MM:SS
        let re_datetime =
            Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();
        if re_datetime.is_match(input) {
            let datetime =
                NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| e.to_string())?;
            return Ok(datetime.and_utc());
        }
        // ISO8601 / RFC3339 格式 (含时区)
        let re_iso = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}$").unwrap();
        if re_iso.is_match(input) {
            let datetime = DateTime::parse_from_rfc3339(input)
                .map_err(|e| e.to_string())?;
            return Ok(datetime.with_timezone(&Utc));
        }
        // ISO8601 / RFC3339 格式 (含时区)
        let re_rfc2822 = Regex::new(
            r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}([+-]\d{2}:\d{2}|Z)$",
        )
        .unwrap();
        if re_rfc2822.is_match(input) {
            let datetime = DateTime::parse_from_rfc2822(input)
                .map_err(|e| e.to_string())?;
            return Ok(datetime.with_timezone(&Utc));
        }
        // RFC3339 兜底
        if let Ok(datetime) = DateTime::parse_from_rfc3339(input) {
            return Ok(datetime.with_timezone(&Utc));
        }
        Err("cannot parse input as datetime".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse_time_string::*;

    #[test]
    fn is_work() {
        let dt = to_datetime("2022-03-25 12:34:56").unwrap();
        println!("Parsed datetime: {}", dt);
    }
}

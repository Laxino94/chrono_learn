pub mod parse_time_string {

    use crate::time_error_type::*;
    use anyhow::Result;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    /// 1. ISO 8601 / RFC 3339 格式
    pub fn parse_rfc3339_date_time(
        s: &str,
    ) -> Result<DateTime<Utc>, time_error_type::DateTimeError> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(time_error_type::DateTimeError::ParseError)
    }
    /// 解析 RFC 2822 格式的时间字符串
    pub fn parse_rfc2822_date_time(
        s: &str,
    ) -> Result<DateTime<Utc>, time_error_type::DateTimeError> {
        DateTime::parse_from_rfc2822(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(time_error_type::DateTimeError::from)
    }

    /// 解析标准日期时间格式 (YYYY-MM-DD HH:MM:SS)
    pub fn parse_standard_date_time(
        s: &str,
    ) -> Result<NaiveDateTime, time_error_type::DateTimeError> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .map_err(time_error_type::DateTimeError::from)
    }

    /// 解析只有日期的字符串 (YYYY-MM-DD)
    pub fn parse_date_only(
        s: &str,
    ) -> Result<NaiveDate, time_error_type::DateTimeError> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(time_error_type::DateTimeError::from)
    }

    /// 解析只有时间的字符串 (HH:MM:SS)
    pub fn parse_time_only(
        s: &str,
    ) -> Result<NaiveTime, time_error_type::DateTimeError> {
        NaiveTime::parse_from_str(s, "%H:%M:%S")
            .map_err(time_error_type::DateTimeError::from)
    }
}

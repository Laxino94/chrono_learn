use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    // 不带时区
    let naive = NaiveDateTime::parse_from_str("2025-09-18 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

    // 转换为 UTC 带时区时间
    let dt_utc: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);
    println!("UTC datetime: {}", dt_utc);
}

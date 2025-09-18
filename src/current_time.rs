#![allow(dead_code)]
pub mod current_time {
    use chrono::{Local, Utc};
    pub fn get_cur_local_date_time() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
    pub fn get_cur_local_date() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }
    pub fn get_cur_local_time() -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    pub fn get_cur_utc_date_time() -> String {
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
    pub fn get_cur_utc_date() -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }
    pub fn get_cur_utc_time() -> String {
        Utc::now().format("%H:%M:%S").to_string()
    }
    pub fn get_cur_timestamp() -> i64 {
        Utc::now().timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use current_time::*;

    #[test]
    fn is_work() {
        println!("Current local time: {}", get_cur_local_date_time());
        println!("Current utc time: {}", get_cur_utc_date_time());
        println!("Current local date: {}", get_cur_local_date());
        println!("Current local time: {}", get_cur_local_time());
        println!("Current utc date: {}", get_cur_utc_date());
        println!("Current utc time: {}", get_cur_utc_time());
        println!("+++++++++++++++++++++++++++++++++++");
        println!("Current utc timestamp: {}", get_cur_timestamp());
    }
}

use chrono::*;

pub fn get_build_ver(end_time: DateTime<Utc>) -> i32{
    let diff_years = end_time.year() - 2019;
    let start_month = 6;
    let end_month = end_time.month() as i32;
    // println!("{} {} {} {}",diff_years, start_month, end_time.year(), end_time.month());
    let build_ver = diff_years * 12 - start_month + end_month + 22;
    return build_ver;
}
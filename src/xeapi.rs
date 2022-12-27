/*
About the Xe Api (also named build version) mod
The origianal start point of the xe api is 2019/1, and it adds weekly.
but in 2019/6, the api version changed to add monthly,
the original cpp code is written in 2020/10, so the algorithm of the 
build version is same as the algorithm after 2019/6.
The hardcoding 22 is present for the beginning of xe xpi.
*/

use chrono::*;

pub fn get_build_ver(end_time: DateTime<Utc>) -> i32{
    // The start time is 2019/6
    let diff_years = end_time.year() - 2019;
    let start_month = 6;
    let end_month = end_time.month() as i32;
    // Calcuate the diff of months
    // println!("{} {} {} {}",diff_years, start_month, end_time.year(), end_time.month());
    let build_ver = diff_years * 12 - start_month + end_month + 22;
    return build_ver;
}
/*---------------------------------------------------------------------
--   ____  _     
--  |  _ \| |      Peter Liebau
--  | |_) | |      17.05.2024
--  |  __/| |___   
--  |_|   |_____|  
---------------------------------------------------------------------*/

use chrono::{DateTime, Local, Utc, TimeZone};
// use std::fs::File;

// ( main )------------------------------------------------------------

fn main() {
    let ts_utc: i32 = 1716174775;
    let timestamp_berlin = ts_utc + 7200;
    let zeit: DateTime<Utc> = Utc.timestamp_opt(timestamp_berlin as i64, 0).unwrap();
    let zeitform = zeit.format("%H:%M:%S");
    println!("Sonnenaufgang {} Uhr", zeitform);
    
// https://rustjobs.dev/blog/date-time-formatting-in-rust/
    let current_local: DateTime<Local> = Local::now();
    // let custom_format = current_local.format("%d.%m.%Y %H:%M:%S");
    let custom_date = current_local.format("%d. %B %Y");
    let custom_time = current_local.format("%H:%M:%S");  
    println!("Heute ist der {} und es ist {} Uhr.", custom_date, custom_time);

    // let custom_dat = current_local.date_naive(); 
    // let custom_ti = current_local.time();
    // println!("{}", custom_dat);
    // println!("{}", custom_ti);

    // let mut file = File::create("foo.txt");
    // file.write_all(b"Hello, world!");

/*    println!("Wie heisst Du?");
    let input = read_string();
    println!("Hallo {}", input);*/
}

/*fn read_string() -> String {
    let mut eingabe = String::new();
    std::io::stdin()
        .read_line(&mut eingabe)
        .expect("can not read user eingabe");
    eingabe
}
*/
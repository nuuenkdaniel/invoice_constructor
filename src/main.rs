use std::io;
use std::env;
// use std::fs;
use std::path::{PathBuf};

use chrono::prelude::*;
use rusqlite::{params, Connection, Result};

// Expected input db: file_path, date: YYYY-MM-DD time_start: hh:mm, time_end: hh:mm
fn input_time(db_path: &str, time_start: &str, time_end: &str, date: Option<&str>,) -> Result<()> {
    let current_date = Local::now().date_naive().to_string();
    let date = date.unwrap_or(&current_date);
    let db_path = PathBuf::from(db_path);
    let conn = Connection::open(&db_path)?;
    conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS tutoring_hours (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          date TEXT NOT NULL CHECK (date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
          time_start TEXT NOT NULL CHECK (time_start GLOB '[0-2][0-9]:[0-5][0-9]'),
          time_end TEXT NOT NULL CHECK (time_end GLOB '[0-2][0-9]:[0-5][0-9]'),
          CHECK (time_end > time_start)
        );
    "#)?;
    conn.execute(
        "INSERT INTO tutoring_hours (date, time_start, time_end) VALUES (?1, ?2, ?3)",
        params![date, time_start, time_end],
    )?;
    Ok(())
}

fn request_string() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong when reading line");
    line
}

fn request_invoice_info() -> (String, String, String, String, f32) {
    println!("Provide the senders name: ");
    let senders_name = request_string();

    println!("Provide the location name: ");
    let location_name = request_string();
    println!("Provide the location street address: ");
    let location_street = request_string();
    println!("Provide the location city, state, and zip: ");
    let location_city = request_string();

    println!("Provide the rate: ");
    let rate: f32 = match request_string().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid rate, please provide a number");
            panic!();
        }
    };
    (senders_name, location_name, location_street, location_city, rate)
}

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 1 {
        let date = if args.len() >= 5 {
            Some(args[4].as_str())
        } else {
            None
        };
        input_time(&args[1], &args[2], &args[3], date)?;
    }
    else {
        request_invoice_info();
    }
    println!("TO BE IMPLEMENTED");
    Ok(())
}

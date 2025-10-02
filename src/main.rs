use std::io;
use std::env;
use std::fs;
use std::path::{PathBuf};

use chrono::prelude::*;
use rusqlite::{params, Connection, Result};

// Expected input db: file_path, date: YYYY-MM-DD time_start: hh:mm, time_end: hh:mm
fn input_time(db_path: String, time_start: String, time_end: String, date: Option<String>,) {
    let date = date.unwrap_or(Local::now().date_naive().to_string());
    let db_path = PathBuf::from(db_path);

    let conn = Connection::open(db_path);
    con.execute_batch(
        "CREATE TABLE IF NOT EXISTS tutoring_time (
            date TEXT,
            time_start TEXT,
            time_end TEXT,
        ",
        (),
        
}

fn request_string() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong when reading line");
    line
}

fn request_invoice_info() -> (String, String, String, String) {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Provide the senders name: ");
    let senders_name = request_string();

    println!("Provide the location name: ");
    let location_name = request_string();
    println!("Provide the location street address: ");
    let location_street = request_string();
    println!("Provide the location city, state, and zip: ");
    let location_city = request_string();

    (senders_name, location_name, location_street, location_city)
}

fn main() {
    request_invoice_info();
    println!("TO BE IMPLEMENTED");
}

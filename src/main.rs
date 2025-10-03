use std::fs;
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

fn request_invoice_info() -> 
(
    String, String, String, String,
    String, String, String, String,
    String, String
) {
    println!("Provide the senders name: ");
    let senders_name = request_string();

    println!("Provide the location name: ");
    let location_name = request_string();
    println!("Provide the location street address: ");
    let location_street = request_string();
    println!("Provide the location city, state, and zip: ");
    let location_city = request_string();

    println!("Provide the parent name: ");
    let parent_name = request_string();
    println!("Provide the student name: ");
    let student_name = request_string();
    println!("Provide the bill to street address: ");
    let bill_to_street = request_string();
    println!("Provide the bill to city, state, and zip: ");
    let bill_to_city = request_string();

    println!("Provide the invoice#: ");
    let invoice_num = match request_string().trim().parse::<usize>() {
        Ok(num) => num.to_string(),
        Err(_) => {
            println!("Invalid number, please provide a number");
            panic!();
        }
    };

    println!("Provide the rate: ");
    let rate = match request_string().trim().parse::<f32>() {
        Ok(num) => num.to_string(),
        Err(_) => {
            println!("Invalid rate, please provide a number");
            panic!();
        }
    };
    (
        senders_name, location_name, location_street, location_city,
        parent_name, student_name, bill_to_street, bill_to_city,
        invoice_num, rate
    )
}

fn generate_tutoring_invoice(
    invoice_info: (
    String, String, String, String,
    String, String, String, String,
    String, String
    ),
    template_path: &str,
    output_path: &str,
) -> Result<()> {
    let template_path = PathBuf::from(template_path);
    let output_path = PathBuf::from(output_path);
    let mut template = fs::read_to_string(template_path).unwrap();

    template = template.replace("{{SENDER_NAME}}", &invoice_info.0.trim());
    template = template.replace("{{LOCATION_NAME}}", &invoice_info.1.trim());
    template = template.replace("{{LOCATION_STREET_ADDRESS}}", &invoice_info.2.trim());
    template = template.replace("{{LOCATION_CITY_STATE_ZIP}}", &invoice_info.3.trim());
    template = template.replace("{{INVOICE_NUMBER}}", &invoice_info.8);
    let date = Local::now().date_naive().to_string();
    template = template.replace("{{DATE}}", &date);
    template = template.replace("{{PARENT_NAME}}", &invoice_info.4.trim());
    template = template.replace("{{STUDENT_NAME}}", &invoice_info.5.trim());
    template = template.replace("{{BILL_TO_STREET_ADDRESS}}", &invoice_info.6.trim());
    template = template.replace("{{BILL_TO_CITY_STATE_ZIP}}", &invoice_info.7.trim());

    fs::write(output_path, template).unwrap();
    Ok(())
}

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 3 {
        let date = if args.len() >= 5 { Some(args[4].as_str()) } else { None };
        input_time(&args[1], &args[2], &args[3], date)?;
    }
    else {
        let invoice_info = request_invoice_info();
        generate_tutoring_invoice(invoice_info, "templates/tutoring_invoice.tex", "test.tex")?;
    }
    println!("TO BE IMPLEMENTED");
    Ok(())
}

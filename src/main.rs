use std::fs;
use std::io;
use std::path::{PathBuf};

use chrono::prelude::*;
use rusqlite::{params, Connection, Result};

#[derive(Default, Debug)]
struct InvoiceInfo {
    title: String,
    sender_name: String,
    location_name: String,
    location_street: String,
    location_city: String,
    parent_name: String,
    student_name: String,
    bill_to_street: String,
    bill_to_city: String,
    invoice_num: String,
    rate: String,
    payment_method: String
}

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

fn request_invoice_info() -> InvoiceInfo {
    println!("Provide a title for the invoice: ");
    let title = request_string();

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

    println!("Provide a payment method: ");
    let payment_method = request_string();

    InvoiceInfo { 
        title: title,
        sender_name: senders_name,
        location_name: location_name,
        location_street: location_street,
        location_city: location_city,
        parent_name: parent_name,
        student_name: student_name,
        bill_to_street: bill_to_street,
        bill_to_city: bill_to_city,
        invoice_num: invoice_num,
        rate: rate,
        payment_method: payment_method
    }
}

struct Hours {
    date: String,
    time_start: String,
    time_end: String,
}

fn generate_tutoring_invoice(
    db_path: &str,
    invoice_info: InvoiceInfo,
    template_path: &str,
    output_path: &str,
) -> Result<()> {
    let template_path = PathBuf::from(template_path);
    let output_path = PathBuf::from(output_path);
    let mut template = fs::read_to_string(template_path).unwrap();

    template = template.replace("{{TITLE}}", &invoice_info.title.trim());
    template = template.replace("{{SENDER_NAME}}", &invoice_info.sender_name.trim());
    template = template.replace("{{LOCATION_NAME}}", &invoice_info.location_name.trim());
    template = template.replace("{{LOCATION_STREET_ADDRESS}}", &invoice_info.location_street.trim());
    template = template.replace("{{LOCATION_CITY_STATE_ZIP}}", &invoice_info.location_city.trim());
    template = template.replace("{{INVOICE_NUMBER}}", &invoice_info.invoice_num.trim());
    let date = Local::now().date_naive().to_string();
    template = template.replace("{{DATE}}", &date);
    template = template.replace("{{PARENT_NAME}}", &invoice_info.parent_name.trim());
    template = template.replace("{{STUDENT_NAME}}", &invoice_info.student_name.trim());
    template = template.replace("{{BILL_TO_STREET_ADDRESS}}", &invoice_info.bill_to_street.trim());
    template = template.replace("{{BILL_TO_CITY_STATE_ZIP}}", &invoice_info.bill_to_city.trim());

    let rate: f32 = invoice_info.rate.trim().parse().expect("Given rate could not be parsed to f32");

    let db_path = PathBuf::from(db_path);
    let conn = Connection::open(&db_path)?;
    let mut stmt = conn.prepare("SELECT date, time_start, time_end FROM tutoring_hours")?;

    // Get hours from db
    let tutoring_hours = stmt.query_map([], |row| {
        Ok(Hours {
            date: row.get(0)?,
            time_start: row.get(1)?,
            time_end: row.get(2)?,
        })
    })?;

    let mut total_hours: f32 = 0.0;
    let mut rows: Vec<String> = Vec::new();
    for hour in tutoring_hours {
        let data = hour?;

        let (mut hrs, mut mins) = data.time_start.split_once(':').expect("Expected as HH:MM format");
        let start_hour: f32 = 
        ((hrs.parse::<f32>().expect("Failed to convert string hours to f32 hours")
            + mins.parse::<f32>().expect("Failed to convert string minutes to f32 minutes")/60.0)*2.0)
            .round()/2 as f32;

        (hrs, mins) = data.time_end.split_once(':').expect("Expected as HH:MM format");
        let end_hour: f32 = 
        ((hrs.parse::<f32>().expect("Failed to convert string hours to f32 hours")
            + mins.parse::<f32>().expect("Failed to convert string minutes to f32 minutes")/60.0)*2.0)
            .round()/2.0 as f32;

        let timedelta = end_hour - start_hour;
        total_hours += timedelta;

        let description = "test";
        let row_formatted: String = format!("{} & {} & {}-{} & {} & {} \\\\", data.date, description, data.time_start, data.time_end, rate, timedelta*rate);
        rows.push(row_formatted);
        println!("{}\t{}\t{}", data.date, data.time_start, data.time_end);
        println!("Hours: {timedelta}");
    }
    println!("Total Hours: {total_hours}");
    let pattern = "% COLUMN START";
    if let Some(index) = template.find(pattern) {
        let mut insertion_point = index + pattern.len();
        for row in rows.iter() {
            let row_formatted: String = format!("\n{}", row);
            template.insert_str(insertion_point, &row_formatted);
            insertion_point += row_formatted.len();
        }
    }
    template = template.replace("{{TOTAL}}", &((total_hours*rate).to_string()));
    template = template.replace("{{PAID}}", &((total_hours*rate).to_string()));
    template = template.replace("{{PAYMENT_METHOD}}", &invoice_info.payment_method.trim());

    fs::write(output_path, template).unwrap();
    Ok(())
}

use clap::Parser;

#[derive(Default, Debug)]
struct ExecValues {
    db_path: String,
    template_path: String,
    output_path: String,
    invoice_info: InvoiceInfo
}

fn parse_exec_config(_config_path: &str) -> ExecValues {

}

fn parse_exec(vals: &[String]) -> ExecValues {
    let mut exec_config = ExecValues::default();
    if vals[0] == "-c" { exec_config = parse_exec_config(&vals[1]); }
    else {
        exec_config.db_path = vals[0].clone();
        exec_config.template_path = vals[1].clone();
        exec_config.output_path = vals[2].clone();
        exec_config.invoice_info = request_invoice_info();
    }
    exec_config
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(
        short = 'i',
        long = "input_time",
        value_names = ["DB_PATH", "START_TIME", "END_TIME", "DATE"],
        num_args = 3..=4
    )]
    input_time: Option<Vec<String>>,

    #[arg(
        short = 'x',
        long = "exec",
        allow_hyphen_values = true,
        value_names = ["DB_PATH", "TEMPLATE_PATH", "OUTPUT_PATH"],
        num_args = 2
    )]
    exec: Option<Vec<String>>,

    #[arg(short = 'c', long="config")] config: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(input_vals) = cli.input_time.as_deref() {
        let date: Option<&str> = input_vals.get(3).map(|x| x.as_str());
        input_time(&input_vals[0], &input_vals[1], &input_vals[2], date)?;
    }
    if let Some(exec_vals) = cli.exec.as_deref() {
        let config: ExecValues = parse_exec(exec_vals);
        generate_tutoring_invoice(config.db_path.as_str(), config.invoice_info, config.template_path.as_str(), config.output_path.as_str())?;
    }

    Ok(())
}

use std::fs;
use std::io;
use std::path::{PathBuf};
use std::error::Error;

use chrono::prelude::*;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
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

fn view_db(db_path: &str) -> Result<()> {
    let db_path = PathBuf::from(db_path);
    let conn = Connection::open(&db_path)?;
    let mut stmt = conn.prepare("SELECT date, time_start, time_end, description FROM tutoring_hours")?;

    // Get hours from db
    let tutoring_hours = stmt.query_map([], |row| {
        Ok(Items {
            date: row.get(0)?,
            time_start: row.get(1)?,
            time_end: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    for item in tutoring_hours {
        let item = item?;
        println!("{}\t{}\t{}\t{}", item.date, item.time_start, item.time_end, item.description);
    }
    Ok(())
}

// Expected input db: file_path, date: YYYY-MM-DD time_start: hh:mm, time_end: hh:mm
fn input_time(db_path: &str, time_start: &str, time_end: &str, description: &str, date: Option<&str>,) -> Result<String> {
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
          description TEXT NOT NULL,
          CHECK (time_end > time_start)
        );
    "#)?;
    conn.execute(
        "INSERT INTO tutoring_hours (date, time_start, time_end, description) VALUES (?1, ?2, ?3, ?4)",
        params![date, time_start, time_end, description],
    )?;
    let formatted_entry = format!("{}\t{}\t{}\t{}", date, time_start, time_end, description);
    Ok(formatted_entry)
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

struct Items {
    date: String,
    time_start: String,
    time_end: String,
    description: String,
}

fn generate_tutoring_invoice( db_path: &str, invoice_info: InvoiceInfo, template_path: &str, output_path: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let template_path = PathBuf::from(template_path);
    let output_path = PathBuf::from(output_path);
    let mut template = fs::read_to_string(template_path)
        .expect("file could not be found or does not exist");

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
    let mut stmt = conn.prepare("SELECT date, time_start, time_end, description FROM tutoring_hours")?;

    // Get hours from db
    let tutoring_hours = stmt.query_map([], |row| {
        Ok(Items {
            date: row.get(0)?,
            time_start: row.get(1)?,
            time_end: row.get(2)?,
            description: row.get(3)?,
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

        // Convert to 12 hr format
        let start_time_formatted: String = convert_12hr(start_hour);

        (hrs, mins) = data.time_end.split_once(':').expect("Expected as HH:MM format");
        let end_hour: f32 = 
        ((hrs.parse::<f32>().expect("Failed to convert string hours to f32 hours")
            + mins.parse::<f32>().expect("Failed to convert string minutes to f32 minutes")/60.0)*2.0)
            .round()/2.0 as f32;

        // Convert to 12 hr format
        let end_time_formatted: String = convert_12hr(end_hour);

        let timedelta = end_hour - start_hour;
        total_hours += timedelta;

        let description = data.description;
        let row_formatted: String = format!("{} & {} & {}-{} & {} & {} \\\\", data.date, description, start_time_formatted, end_time_formatted, rate, timedelta*rate);
        println!("{row_formatted}");
        rows.push(row_formatted);
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

    fs::write(output_path.clone(), template).unwrap();
    Ok(output_path)
}

fn convert_12hr(hrs: f32) -> String {
    let hours: i64 = hrs.floor().round() as i64;
    let minutes: i64 = ((hrs%1.0)*60.0).round() as i64;
    let minutes_formatted: String =  
    if minutes < 10 { format!("0{}", minutes) }
    else { minutes.to_string() };
    if hours == 12 {
        format!("12:{}pm", minutes_formatted)
    }
    else if hours == 0 {
        format!("12:{}am", minutes_formatted)
    }
    else if hours > 12 {
        format!("{}:{}pm", hours-12, minutes_formatted)
    }
    else {
        format!("{}:{}am", hours, minutes_formatted)
    }
}

// Argument Parsing Functions
use clap::Parser;

#[derive(Default, Debug, Deserialize, Serialize)]
struct ExecValues {
    db_path: String,
    template_path: String,
    output_path: String,
    invoice_info: InvoiceInfo
}

fn parse_exec(vals: &[String]) -> ExecValues {
    let mut exec_config = ExecValues::default();
    if vals[0] == "-c" {
        let conf_json = fs::read_to_string(vals[1].as_str())
            .expect("Something went wrong when reading the config file");
        exec_config = serde_json::from_str(&conf_json)
            .expect("Something went wrong when parsing the file config file");
    }
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
        value_names = ["DB_PATH", "START_TIME", "END_TIME", "DESCRIPTION", "DATE"],
        num_args = 4..=5
    )]
    input_time: Option<Vec<String>>,

    #[arg(
        short = 'x',
        long = "exec",
        allow_hyphen_values = true,
        value_names = ["DB_PATH", "TEMPLATE_PATH", "OUTPUT_PATH"],
        num_args = 2..=3
    )]
    exec: Option<Vec<String>>,

    #[arg(short = 'c', long="config")] config: Option<String>,
    #[arg(short = 'v', long="view_db")] view_db: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(input_vals) = cli.input_time.as_deref() {
        let date: Option<&str> = input_vals.get(4).map(|x| x.as_str());
        let db_entry:String = input_time(&input_vals[0], &input_vals[1], &input_vals[2], &input_vals[3],  date)?;
        println!("Added entry: {}", db_entry);
    }
    if let Some(exec_vals) = cli.exec.as_deref() {
        let config: ExecValues = parse_exec(exec_vals);
        let output_path: PathBuf = generate_tutoring_invoice(config.db_path.as_str(), config.invoice_info, config.template_path.as_str(), config.output_path.as_str()).unwrap();
        println!("Generated: {}", output_path.display());
    }
    if let Some(view_db_vals) = cli.view_db.as_deref() {
        view_db(&view_db_vals)?;
    }
    Ok(())
}

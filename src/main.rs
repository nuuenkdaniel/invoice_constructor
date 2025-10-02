use std::io;

fn request_string() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong when reading line");
    line
}

fn _request_invoice_info() -> (String, String, String, String) {
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
    println!("TO BE IMPLEMENTED");
}

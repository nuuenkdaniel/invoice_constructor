use std::io;

fn request_string() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong");
    println!("Got {line}");
    line
}

fn main() {
    println!("Provide the senders name: ");
    let _senders_name = request_string();
    
    println!("Provide the location: ");
    let _location = request_string();
}

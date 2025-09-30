use std::io;

fn request_data(data: &mut String) {
    io::stdin()
        .read_line(data)
        .expect("Something went wrong");
    println!("Got {data}");
}

fn main() {
    println!("Provide the senders name: ");
    let mut senders_name = String::new();
    request_data(&mut senders_name);
    let mut location = String::new();
    request_data(&mut location);
}

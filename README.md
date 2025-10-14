# invoice_constructor
CLI tool to quickly construct an invoice and input time to a database to be used by the constructor

## Installation
### Download a prebuilt binary (recommended)
1. Go to **Releases**.
2. Download the binary from the archive that matches your system
3. Move the downloaded binary to where you want it
```bash
# Linux/macOS example
sudo mv invoicegen /usr/local/bin/
invoicegen --version
```

### Building from latest release
```bash
# Prerequisites
# You need to have the Rust tool chain

git clone https://github.com/nuuenkdaniel/invoice_constructor.git
cd invoice_constructor 
cargo build --release
sudo mv target/release/invoice_constructor /usr/local/bin/
```

## How to use
- Inputing time to the db
```bash
invoice_constructor -i <DB_PATH> <START_TIME> <END_TIME> <DATE>
```

- Generating invoice from the database
```bash
-x <DB_PATH> <TEMPLATE_PATH> <OUTPUT_PATH>
```

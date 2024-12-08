# IP Port Scanner

A fast, concurrent port scanner written in Rust that leverages async/await for efficient network scanning.

## Features

- Asynchronous port scanning using Tokio
- Customizable port range scanning
- IPv4 and IPv6 support
- Simple and clean command-line interface
- Progress indication during scanning
- Sorted output of open ports

## Installation

To install the port scanner, you'll need to have Rust and Cargo installed on your system. Then:

```bash
# Clone the repository
git clone https://github.com/sanjaykohli/ip_sniff.git

# Change into the project directory
cd ip_sniff

# Build the project
cargo build --release
```

## Usage

```bash
# Basic usage with default settings (scans localhost:1-65535)
cargo run

# Scan specific address
cargo run -- --address 192.168.1.1
# Or using short form
cargo run -- -a 192.168.1.1

# Scan specific port range
cargo run -- --start 80 --end 443
# Or using short form
cargo run -- -s 80 -e 443

# Scan specific address and port range
cargo run -- -a 192.168.1.1 -s 20 -e 1024

# After building with cargo build --release:
./target/release/ip_sniff -a 192.168.1.1 -s 20 -e 1024
```

### Command Line Arguments

- `-a, --address`: Target IP address (default: 127.0.0.1)
- `-s, --start`: Starting port number (must be greater than 0, default: 1)
- `-e, --end`: Ending port number (must be less than or equal to 65535, default: 65535)

## Output

The scanner provides real-time feedback:
- Dots (.) indicate progress during scanning
- Final results show a sorted list of open ports

Example output:
```
Address: 192.168.1.1
Start Port: 1
End Port: 1024
....
Port 22 is open
Port 80 is open
Port 443 is open
```

## Dependencies

Key dependencies from Cargo.toml:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
bpaf = "0.9"
```

## Build Requirements

- Rust 1.54 or higher (recommended due to Tokio requirements)
- Cargo package manager

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

[Sanjay Kohli](https://github.com/sanjaykohli)

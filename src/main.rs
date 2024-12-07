use bpaf::*;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;
use std::io::Write;
// Max IP Port
const MAX: u16 = 65535;

// Address fallback
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// Struct for parsed arguments
#[derive(Debug, Clone)]
pub struct Arguments {
    pub address: IpAddr,
    pub start_port: u16,
    pub end_port: u16,
}

// Function to build the argument parser
pub fn arguments() -> OptionParser<Arguments> {
    // Address argument
    let address = long("address")
        .short('a')
        .argument("Address")
        .fallback(IPFALLBACK);

    // Start port argument
    let start_port = long("start")
        .short('s')
        .argument("Start Port")
        .guard(|&x| x > 0, "Start port must be greater than 0")
        .fallback(1u16);

    // End port argument
    let end_port = long("end")
        .short('e')
        .argument("End Port")
        .guard(|&x| x <= MAX, "End port must be less than or equal to 65535")
        .fallback(MAX);

    // Combine all arguments into the `Arguments` struct
    construct!(Arguments {
        address,
        start_port,
        end_port
    })
    .to_options()
}

// Function to scan a port
async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            std::io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    // Parse the CLI arguments
    let opts = arguments().run();

    println!("Address: {}", opts.address);
    println!("Start Port: {}", opts.start_port);
    println!("End Port: {}", opts.end_port);

    // Initialize the channel
    let (tx, rx) = channel();

    // Iterate through ports and spawn a task for each
    for port in opts.start_port..=opts.end_port {
        let tx = tx.clone();
        let addr = opts.address;

        task::spawn(async move {
            scan(tx, port, addr).await;
        });
    }

    // Collect and print open ports
    let mut out = vec![];
    drop(tx);

    for port in rx {
        out.push(port);
    }

    println!();
    out.sort_unstable();
    for port in out {
        println!("Port {} is open", port);
    }
}

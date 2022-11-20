use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

// Max IP Port.
const MAX: u16 = 65535;

// Address fallback.
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// CLI Arguments.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    // Address argument.  Accepts -a and --address and an IpAddr type. Falls back to the above constant.
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    pub address: IpAddr,
}

// Scan the port.
async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    // Attempts Connects to the address and the given port.
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        // If the connection is successful, print out a . and then pass the port through the channel.
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        // If the connection is unsuccessful, do nothing. Means port is not open.
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    // collect the arguments.
    let opts = arguments().run();
    // Initialize the channel.
    let (tx, rx) = channel();
    // Iterate through all of the ports so that we can spawn a single task for each.
    // (Much faster than before because it uses green threads instead of OS threads.)
    for i in 1..MAX {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, i, opts.address).await });
    }
    // Create the vector for all of the outputs.
    let mut out = vec![];
    // Drop the tx clones.
    drop(tx);
    // Wait for all of the outputs to finish and push them into the vector.

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        // Iterate through the outputs and print them out as being open.
        println!("{} is open", v);
    }
}

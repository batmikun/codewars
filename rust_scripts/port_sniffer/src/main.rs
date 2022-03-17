use std::io::{self, Write};
use std::net::TcpStream;
use std::{env, net::IpAddr, str::FromStr, process, thread};
use std::sync::mpsc::{Sender, channel};

const MAX: u16 = 65535; // max port we can sniff

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 { // This if else checks that the number of arguments is correct
            return Err("Not enoguht arguments")
        } else if args.len() > 4 {
            return Err("Too many arguments")
        }

        let f = args[1].clone(); // Clone the first argument that follows the name of the program 

        if let Ok(ipaddr) = IpAddr::from_str(&f) { // Check if that first argument is a IPADDR
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4}) // If is a ipaddr we return the struct
        } else {
            let flag = args[1].clone(); // Because we know that is not an ippadr has to be a flag, first we check if it's the help flag
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how mant threads you want");
                println!("-h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") { // If it's not the help flag it's the j flag
                let ipaddr = match IpAddr::from_str(&args[3]) { // check if the ipaddr is valid
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid ippadr, must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>(){ // parse the number of threads from string to u16
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };

                return Ok(Arguments{threads, flag, ipaddr}); // create new argument struct
            } else {
                return Err("Invalid Sintax");
            }
        }
    }
}

fn scan( tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) { // Here we check if the port is open
            Ok(_) => {
                print!("."); // If its open we return something to the console
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        
        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // This collects the arguments that we write when we execute a binary
    let program = args[0].clone(); // Extract name of the program
    let arguments = Arguments::new(&args).unwrap_or_else(|err| { // we call our new function and unwrap to handle the error if there is any that we create
        if err.contains("help") {
            process::exit(0); // This will exit our program without panic
        } else {
            eprintln!("{} problem parsing the arguments: {}", program, err);
            process::exit(0);
        }
    });

    let addr = arguments.ipaddr;
    let num_threads = arguments.threads;
    let (tx, rx) = channel(); // Here we obtain a Sender and Receiver

    for i in 0..num_threads { // We create a new thread base on the num_threads
        let tx = tx.clone(); // clone the Sender so each thread has its own sender

        thread::spawn(move || { // spaws a need thread
            scan(tx, i, addr, num_threads); 
        });
    }

    drop(tx);

    let mut out = vec![];
    for p in rx {
        out.push(p);
    }
    out.sort();

    println!("");
    
    for port in out {
        println!("{} is open", port);
    }
}


/*
    Valid inputs:
        script -flag ip

        -h == help
        -j number == number of threads

        1 - port_sniffer.exe -h
        2 - port_sniffer.exe -j 100 127.0.0.1
        3 - port_sniffer.exe 127.0.0.1
*/
mod scanner;
mod models;
mod utils;

use scanner::Scanner;

fn main() {

    let host = "127.0.0.1";      // Immutable variables
    let start_port: u16 = 1;
    let end_port: u16 = 1024;

    let scanner = Scanner::new(300, 128); // Struct + impl (OO technique)

    println!("Scanning {host} ports {start_port}-{end_port}...\n");


    let findings = scanner.scan_range(host, start_port, end_port); // Vec returned (data structure)


    if findings.is_empty() {     // Conditional
        println!("No open TCP ports found in range.");
        return;
    }

    println!("Open ports found: {}", findings.len());
    println!("----------------------------------");
    for f in findings {
        match f.banner {
            Some(b) if !b.is_empty() => println!("Port {:5} | Banner: {}", f.port, b),    // Loop
            _ => println!("Port {:5} | Banner: (none)", f.port),
        }
    }
}

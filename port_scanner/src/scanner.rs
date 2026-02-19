use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::models::PortFinding;
use crate::utils::resolve_first;

pub struct Scanner {
    timeout: Duration,
    banner_bytes: usize,
}

impl Scanner {
    pub fn new(timeout_ms: u64, banner_bytes: usize) -> Self {
        Self {
            timeout: Duration::from_millis(timeout_ms),
            banner_bytes,
        }
    }


    fn scan_port(&self, host: &str, port: u16) -> Option<PortFinding> {  // Borrowed params: demonstrates references (&str) rather than taking ownership (String)
        let addr = resolve_first(host, port)?; 

        let stream = TcpStream::connect_timeout(&addr, self.timeout);
        if stream.is_err() {
            return None;
        }
        let mut stream = stream.unwrap();

        let _ = stream.set_read_timeout(Some(self.timeout));
        let _ = stream.set_write_timeout(Some(self.timeout));

     
        let _ = stream.write_all(b"\r\n");// Try to coax a banner (harmless newline); some services won't respond.

        let mut buf = vec![0u8; self.banner_bytes]; // Vec + mutable variable
        let n = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(_) => 0,
        };

        let banner = if n > 0 {
            Some(String::from_utf8_lossy(&buf[..n]).trim().to_string())
        } else {
            None
        };

        Some(PortFinding { port, banner })
    }

    pub fn scan_range(&self, host: &str, start: u16, end: u16) -> Vec<PortFinding> {
        let mut open_ports: Vec<PortFinding> = Vec::new();

        for port in start..=end {
            if let Some(finding) = self.scan_port(host, port) {
                open_ports.push(finding);
            }
        }

        open_ports
    }
}

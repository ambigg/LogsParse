use crate::LogEntry;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::error::Error;
use std::fs;
use std::net::IpAddr;

// pub struct LogEntry {
//     pub ip: IpAddr,
//     pub timestamp: DateTime<Utc>,
//     pub method: String,
//     pub url: String,
//     pub status: Option<u32>,
//     pub size: Option<u32>,
// }

pub fn parse(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut logs: Vec<LogEntry> = Vec::new();
    for line in contents.lines() {
        if let Some(entry) = parse_log(line) {
            logs.push(entry);
        }
    }
    Ok(logs)
}
//this function parse each line it gets
fn parse_log(line: &str) -> Option<LogEntry> {
    let re = Regex::new(
        r#"^(\d+\.\d+\.\d+\.\d+) - - \[([^\]]+)\] "(\w+) ([^"]+) HTTP/[\d\.]+" (\d+) (\d+)$"#,
    )
    .unwrap();
    if let Some(caps) = re.captures(line) {
        Some(LogEntry {
            ip: parse_ip(&caps[1])?,
            timestamp: parse_timestamp(&caps[2])?,
            method: caps[3].to_string(),
            url: parse_url(&caps[4]),
            status: parse_status(&caps[5]),
            size: parse_size(&caps[6]),
        })
    } else {
        None
    }
}

fn parse_ip(ip_str: &str) -> Option<IpAddr> {
    match ip_str.parse() {
        Ok(ip) => Some(ip),
        Err(e) => {
            eprintln!("Error parsing IP: {}", e);
            None
        }
    }
}

fn parse_timestamp(timestamp_str: &str) -> Option<DateTime<Utc>> {
    // [10/Jan/2024:14:30:45 +0000]
    DateTime::parse_from_str(timestamp_str, "%d/%b/%Y:%H:%M:%S %z")
        .map(|dt| dt.with_timezone(&Utc))
        .inspect_err(|e| eprintln!("Error parsing TIMESTAMP {}: {}", timestamp_str, e))
        .ok()
}
    
//Is not well parsed the url is the same in string 
fn parse_url(url_str: &str) -> String {
    url_str.to_string()
}

fn parse_status(status_str: &str) -> Option<u32> {
    match status_str.parse() {
        Ok(status) => Some(status),
        Err(e) => {
            eprintln!("Error parsing STATUS: {}", e);
            None
        }
    }
}

fn parse_size(size_str: &str) -> Option<u32> {
    match size_str.parse() {
        Ok(size) => Some(size),
        Err(e) => {
            eprintln!("Error parsing SIZE: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;


    #[test]
    fn test_parse_log() {
        let line =
            "192.168.1.100 - - [10/Jan/2024:14:30:45 +0000] \"GET /home.html HTTP/1.1\" 200 2048";
        let result = parse_log(line);

        assert!(result.is_some());
        let entry = result.unwrap();

        // Verificar IP
        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));

        // Verificar timestamp
        let expected_timestamp =
            DateTime::parse_from_str("10/Jan/2024:14:30:45 +0000", "%d/%b/%Y:%H:%M:%S %z")
                .unwrap()
                .with_timezone(&Utc);
        assert_eq!(entry.timestamp, expected_timestamp);

        // Verificar método
        assert_eq!(entry.method, "GET");

        // Verificar URL
        assert_eq!(entry.url, "/home.html");

        // Verificar status
        assert_eq!(entry.status, Some(200));

        // Verificar size
        assert_eq!(entry.size, Some(2048));
    }
}

//here gets the entry se parated and parsed and realize the counting and organization of the data

//
// //  LogEntry {
//         ip: 192.168.1.105,
//         timestamp: 2024-01-10T14:39:00Z,
//         method: "GET",
//         url: "/noticias.html",
//         status: Some(
//             200,
//         ),
//         size: Some(
//             1024,
//         ),
//     },
// ]
//TODO: this is this
use std::collections::HashMap;
use std::error::Error;
use std::net::IpAddr;

use chrono::{DateTime, Utc};

use crate::LogEntry;
use crate::LogStat;

//count total request
//identify more common HTTP request
//Detect top IPS
//Detect top URL
//
//more busy time

pub fn analize(log_entry: Vec<LogEntry>) -> Result<LogStat, Box<dyn Error>> {
    let mut stats = LogStat {
        total_requests: 0,
        top_http_status: HashMap::new(),
        top_ips: Vec::new(),
        top_urls: Vec::new(),
        methods: HashMap::new(),
        total_bytes: 0,
        lineswerrors: Vec::new(),
    };

    let mut ip_counts: HashMap<String, usize> = HashMap::new();
    let mut url_counts: HashMap<String, usize> = HashMap::new();

    for entry in log_entry {
        //count total request
        stats.total_requests += 1;

        //count methods http
        *stats.methods.entry(entry.method.clone()).or_insert(0) += 1;

        //count http status
        if let Some(status) = entry.status {
            *stats.top_http_status.entry(status).or_insert(0) += 1;

            if status >= 400 {
                stats.lineswerrors.push(entry.clone());
            }
        }

        //count byte total
        if let Some(size) = entry.size {
            stats.total_bytes += size as u64;
        }

        //count IPs
        let ip_str = entry.ip.to_string();
        *ip_counts.entry(ip_str).or_insert(0) += 1;

        //count URLs
        *url_counts.entry(entry.url.clone()).or_insert(0) += 1;
    }

    let mut ip_vec: Vec<(String, usize)> = ip_counts.into_iter().collect();
    ip_vec.sort_by(|a, b| b.1.cmp(&a.1));
    stats.top_ips = ip_vec.into_iter().take(10).collect();

    let mut url_vec: Vec<(String, usize)> = url_counts.into_iter().collect();
    url_vec.sort_by(|a, b| b.1.cmp(&a.1));
    stats.top_urls = url_vec.into_iter().take(10).collect();

    Ok(stats)
}
// pub struct LogStat {
//     pub total_requests:usize,
//     pub status_codes:HashMap<u32,usize>,
//     pub top_ips: Vec<(String,usize)>,
//     pub top_urls:Vec<(String,usize)>,
//     pub methods:HashMap<String,usize>,
//     pub total_bytes:u64,
//     pub lineserrors: Vec<LogEntry>,
// }


// pub fn get_error_rate(stats: &LogStat) -> f64 {
//
//     if stats.lineswerrors = 0{
//         return 0.0;
//     }
//
//
// }
//

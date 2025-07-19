//here run all the other files so the main only have to run the lib file and lib handles everything
//else
use chrono::{DateTime, Utc};
use std::{collections::HashMap, error::Error, net::IpAddr, process};

use crate::parser::parse;
use crate::analizer::analize;
use crate::output::clean;

pub mod parser;
pub mod analizer;
pub mod output;

#[derive(Debug,Clone)]
pub struct LogEntry {
    pub ip: IpAddr,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub status: Option<u32>,
    pub size: Option<u32>,
}
#[derive(Debug,Clone)]
pub struct LogStat {
    pub total_requests:usize,
    pub top_http_status:HashMap<u32,usize>,
    pub top_ips: Vec<(String,usize)>,
    pub top_urls:Vec<(String,usize)>,
    pub methods:HashMap<String,usize>,
    pub total_bytes:u64,
    pub lineswerrors: Vec<LogEntry>,
}
#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub status_codes: Option<Vec<u32>>,
    pub status_range: Option<(u32, u32)>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub ip_filter: Option<String>,
    pub url_pattern: Option<String>,
    pub methods: Option<Vec<String>>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub errors_only: bool,
    pub last_hours: Option<u32>,
}

#[derive(Debug)]
pub struct Config {
    pub def: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Is not a logsparse entry");
        }
         let def = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { def, file_path })
    }
}
//here the lib run each stage of the project
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let logentry_parsed:Vec<LogEntry> = parse(&config.file_path)?;
    println!("{:#?}",logentry_parsed);
    let logstat_analized: LogStat = analize(logentry_parsed)?;
    println!("{:#?}",logstat_analized);

    // let clean_output= clean(&logstat_analized,&config.)?;
    // println!("{:#?}",clean_output);
    Ok(())
}

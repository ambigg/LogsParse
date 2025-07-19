//here is to config the command using clap, not a priority

//TODO:
//No filter
//filter for STATUS
//filter for DATE START - DATE FINISH
//filter for LINES WITH ERROR
//filter for 
//
// pub struct FilterOptions {
//     pub status_codes: Option<Vec<u32>>,
//     pub status_range: Option<(u32, u32)>,
//     pub date_from: Option<DateTime<Utc>>,
//     pub date_to: Option<DateTime<Utc>>,
//     pub ip_filter: Option<String>,
//     pub url_pattern: Option<String>,
//     pub methods: Option<Vec<String>>,
//     pub min_size: Option<u64>,
//     pub max_size: Option<u64>,
//     pub errors_only: bool,
//     pub last_hours: Option<u32>,
// }


use clap::Parser;

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]





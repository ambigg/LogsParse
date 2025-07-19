//this one just inicialice the proyect
use std::{env, process};

use logsparse::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments, {}", err);
        process::exit(1);
    });
    println!("{:?}", config);

    if let Err(e)= logsparse::run(config){
        println!("Error on run function: {}",e);
        process::exit(1);
    }
}

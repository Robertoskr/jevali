use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use serde_json::{Result, Value};
mod jevalis_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let jevalis_file = File::open(args[1].clone()).expect("Cant open jevali file");
    let json_file = File::open(args[2].clone()).expect("Cant open json file");

    //create the readers for each file 
    let mut json_reader = BufReader::new(json_file);
    let mut jevalis_reader = BufReader::new(jevalis_file);
    

    //pre process the two files 
    let mut jevalis_lines = jevalis_reader.lines().map(|line| { line.unwrap() }).collect();
    let mut value: Value = serde_json::from_reader(json_reader).expect("Failed to decode json file");

    //parse the jevalis file 
    let mut jevalis_parser = JevalisParser::new(&jevalis_lines);
    println!("{}", value);
    
}

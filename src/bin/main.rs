use std::io;
use planner::Terminator;
use planner::process;
use std::env;

fn main() {
	let mut running = true;
	while running == true{
        let mut useful_input: Vec<&str>;
        let mut input = String::new();
		io::stdin().read_line(&mut input); 
	    useful_input = input.split_whitespace().collect();
		let mut return_value = process(useful_input,"Save_File.txt".to_string());
		match return_value {
			Ok(Terminator::Terminate) => {
				running=false
			},
			Err(_) => {
				println!("there has been an error in processing your request");
			},
			Ok(Terminator::No) => {},
		}
		
	   	
	}
}
				
				
